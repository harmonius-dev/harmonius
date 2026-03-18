# User Stories — 7.1 Navigation

## F-7.1.1 — Recast-Style NavMesh Generation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.1.1  | designer (P-5)          | F-7.1.1  | R-7.1.1      |
| US-7.1.1.2  | designer (P-5)          | F-7.1.1  | R-7.1.1      |
| US-7.1.1.3  | designer (P-5)          | F-7.1.1  | R-7.1.1      |
| US-7.1.1.4  | player (P-23)           | F-7.1.1  | R-7.1.1      |
| US-7.1.1.5  | player (P-23)           | F-7.1.1  | R-7.1.1      |
| US-7.1.1.6  | player (P-23)           | F-7.1.1  | R-7.1.1      |
| US-7.1.1.7  | engine developer (P-26) | F-7.1.1  | R-7.1.1      |
| US-7.1.1.8  | engine developer (P-26) | F-7.1.1  | R-7.1.1      |
| US-7.1.1.9  | engine developer (P-26) | F-7.1.1  | R-7.1.1      |
| US-7.1.1.10 | engine tester (P-27)    | F-7.1.1  | R-7.1.1      |
| US-7.1.1.11 | engine tester (P-27)    | F-7.1.1  | R-7.1.1      |
| US-7.1.1.12 | engine tester (P-27)    | F-7.1.1  | R-7.1.1      |

1. **US-7.1.1.1** — I want to generate a navigation mesh from static level geometry with one click
   in the editor
   - **Acceptance:** AI agents can pathfind without manual navmesh authoring
2. **US-7.1.1.2** — I want to set the agent radius and height used for NavMesh generation
   - **Acceptance:** the mesh accounts for my character's physical size
3. **US-7.1.1.3** — I want to preview which surfaces are classified as walkable before committing to
   a full NavMesh bake
   - **Acceptance:** I can adjust geometry or slope limits early
4. **US-7.1.1.4** — I want enemies to navigate around furniture, rocks, and other obstacles smoothly
   - **Acceptance:** AI movement feels natural and intelligent
5. **US-7.1.1.5** — I want NPCs to walk on roads and bridges instead of clipping through terrain
   - **Acceptance:** the world feels believable
6. **US-7.1.1.6** — I want large creatures to avoid narrow doorways they cannot fit through
   - **Acceptance:** their movement matches their visible size
7. **US-7.1.1.7** — I want to implement voxel-based NavMesh generation using heightfield
   rasterization
   - **Acceptance:** complex geometry produces accurate walkable surfaces
8. **US-7.1.1.8** — I want to expose slope angle and step-climb parameters to the voxelization
   pipeline
   - **Acceptance:** different agent archetypes get appropriately filtered meshes
9. **US-7.1.1.9** — I want to integrate NavMesh baking into the offline content pipeline
   - **Acceptance:** shipping builds include pre-baked meshes without runtime generation cost
10. **US-7.1.1.10** — I want to verify that generated NavMesh covers 100% of walkable surfaces on a
    reference test level
    - **Acceptance:** no valid paths are missing
11. **US-7.1.1.11** — I want to confirm that surfaces exceeding the configured slope limit are
    excluded from the NavMesh
    - **Acceptance:** agents never path up impassable inclines
12. **US-7.1.1.12** — I want to measure NavMesh generation time on reference levels across platforms
    - **Acceptance:** I can detect performance regressions in the bake pipeline. ---

## F-7.1.2 — NavMesh Tiling & Streaming

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.2.1  | designer (P-5)          | F-7.1.2  | R-7.1.2      |
| US-7.1.2.2  | designer (P-5)          | F-7.1.2  | R-7.1.2      |
| US-7.1.2.3  | designer (P-5)          | F-7.1.2  | R-7.1.2      |
| US-7.1.2.4  | player (P-23)           | F-7.1.2  | R-7.1.2      |
| US-7.1.2.5  | player (P-23)           | F-7.1.2  | R-7.1.2      |
| US-7.1.2.6  | player (P-23)           | F-7.1.2  | R-7.1.2      |
| US-7.1.2.7  | engine developer (P-26) | F-7.1.2  | R-7.1.2      |
| US-7.1.2.8  | engine developer (P-26) | F-7.1.2  | R-7.1.2      |
| US-7.1.2.9  | engine developer (P-26) | F-7.1.2  | R-7.1.2      |
| US-7.1.2.10 | engine tester (P-27)    | F-7.1.2  | R-7.1.2      |
| US-7.1.2.11 | engine tester (P-27)    | F-7.1.2  | R-7.1.2      |
| US-7.1.2.12 | engine tester (P-27)    | F-7.1.2  | R-7.1.2      |

1. **US-7.1.2.1** — I want to configure the NavMesh tile size to align with my world streaming grid
   - **Acceptance:** navigation tiles load and unload with world chunks
2. **US-7.1.2.2** — I want to test that agents path seamlessly across tile boundaries without
   pausing
   - **Acceptance:** the tiled structure is invisible to players
3. **US-7.1.2.3** — I want to set how far ahead NavMesh tiles are preloaded around active agents
   - **Acceptance:** pathfinding never waits on tile I/O
4. **US-7.1.2.4** — I want NPCs to navigate across the open world without stopping at invisible
   boundaries
   - **Acceptance:** travel feels continuous
5. **US-7.1.2.5** — I want NPC navigation to be unaffected when new areas stream in
   - **Acceptance:** pathfinding does not visibly stall during exploration
6. **US-7.1.2.6** — I want AI companions to follow me across large maps without losing their path
   - **Acceptance:** long journeys feel reliable
7. **US-7.1.2.7** — I want to load and unload NavMesh tiles asynchronously as the simulation window
   moves
   - **Acceptance:** the full mesh is never required in memory
8. **US-7.1.2.8** — I want to ensure NavMesh tile boundaries match or subdivide the world chunk grid
   - **Acceptance:** streaming events are synchronized
9. **US-7.1.2.9** — I want to maintain a connectivity graph across tile boundaries
   - **Acceptance:** cross-tile pathfinding produces continuous corridors
10. **US-7.1.2.10** — I want to verify that NavMesh tiles load and unload correctly under
    constrained memory conditions
    - **Acceptance:** no tiles leak or fail to load
11. **US-7.1.2.11** — I want to confirm that pathfinding corridors crossing tile boundaries produce
    valid continuous paths
    - **Acceptance:** tile seams never break navigation
12. **US-7.1.2.12** — I want to profile the I/O bandwidth consumed by NavMesh tile streaming
    - **Acceptance:** it stays within the budget allocated by the streaming system. ---

## F-7.1.3 — A* Pathfinding on NavMesh

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.3.1  | designer (P-5)          | F-7.1.3  | R-7.1.3      |
| US-7.1.3.2  | designer (P-5)          | F-7.1.3  | R-7.1.3      |
| US-7.1.3.3  | designer (P-5)          | F-7.1.3  | R-7.1.3      |
| US-7.1.3.4  | player (P-23)           | F-7.1.3  | R-7.1.3      |
| US-7.1.3.5  | player (P-23)           | F-7.1.3  | R-7.1.3      |
| US-7.1.3.6  | player (P-23)           | F-7.1.3  | R-7.1.3      |
| US-7.1.3.7  | engine developer (P-26) | F-7.1.3  | R-7.1.3      |
| US-7.1.3.8  | engine developer (P-26) | F-7.1.3  | R-7.1.3      |
| US-7.1.3.9  | engine developer (P-26) | F-7.1.3  | R-7.1.3      |
| US-7.1.3.10 | engine tester (P-27)    | F-7.1.3  | R-7.1.3      |
| US-7.1.3.11 | engine tester (P-27)    | F-7.1.3  | R-7.1.3      |
| US-7.1.3.12 | engine tester (P-27)    | F-7.1.3  | R-7.1.3      |

1. **US-7.1.3.1** — I want to assign traversal costs to area types like roads, swamps, and lava
   - **Acceptance:** AI agents prefer contextually appropriate routes
2. **US-7.1.3.2** — I want to place an agent on a map with roads and swamps and verify it prefers
   the road
   - **Acceptance:** area-cost weights work as expected
3. **US-7.1.3.3** — I want to configure the per-tick CPU budget for pathfinding queries
   - **Acceptance:** I can balance path responsiveness against frame rate
4. **US-7.1.3.4** — I want enemies to take logical routes using roads when available and avoiding
   hazards
   - **Acceptance:** AI movement looks purposeful
5. **US-7.1.3.5** — I want AI agents to start moving quickly after receiving a new destination
   - **Acceptance:** there is no visible delay
6. **US-7.1.3.6** — I want NPCs to avoid walking through lava or toxic areas when a safe path exists
   - **Acceptance:** their behavior appears survival-aware
7. **US-7.1.3.7** — I want to implement A* search over NavMesh polygon graphs with configurable
   heuristics
   - **Acceptance:** pathfinding is efficient and extensible
8. **US-7.1.3.8** — I want to batch pathfinding queries and spread them across frames
   - **Acceptance:** CPU cost stays within the per-tick budget
9. **US-7.1.3.9** — I want to allow custom heuristic functions for A* search
   - **Acceptance:** different game scenarios can tune path quality vs. search speed
10. **US-7.1.3.10** — I want to verify that A* produces shortest-cost paths on known reference
    graphs
    - **Acceptance:** the search algorithm is correct
11. **US-7.1.3.11** — I want to confirm that pathfinding never exceeds the configured per-tick CPU
    budget
    - **Acceptance:** frame rate remains stable under heavy query load
12. **US-7.1.3.12** — I want to stress test with 128+ concurrent path queries on desktop and verify
    correct results
    - **Acceptance:** the batching system handles high load. ---

## F-7.1.4 — String Pulling (Funnel Algorithm)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.4.1  | designer (P-5)          | F-7.1.4  | R-7.1.4      |
| US-7.1.4.2  | designer (P-5)          | F-7.1.4  | R-7.1.4      |
| US-7.1.4.3  | designer (P-5)          | F-7.1.4  | R-7.1.4      |
| US-7.1.4.4  | player (P-23)           | F-7.1.4  | R-7.1.4      |
| US-7.1.4.5  | player (P-23)           | F-7.1.4  | R-7.1.4      |
| US-7.1.4.6  | player (P-23)           | F-7.1.4  | R-7.1.4      |
| US-7.1.4.7  | engine developer (P-26) | F-7.1.4  | R-7.1.4      |
| US-7.1.4.8  | engine developer (P-26) | F-7.1.4  | R-7.1.4      |
| US-7.1.4.9  | engine developer (P-26) | F-7.1.4  | R-7.1.4      |
| US-7.1.4.10 | engine tester (P-27)    | F-7.1.4  | R-7.1.4      |
| US-7.1.4.11 | engine tester (P-27)    | F-7.1.4  | R-7.1.4      |
| US-7.1.4.12 | engine tester (P-27)    | F-7.1.4  | R-7.1.4      |

1. **US-7.1.4.1** — I want to see agents follow tight, corner-hugging waypoints in the debug overlay
   - **Acceptance:** I can confirm the funnel algorithm produces clean paths
2. **US-7.1.4.2** — I want to toggle between raw polygon corridor and funnel-smoothed paths in the
   editor
   - **Acceptance:** I can visualize the improvement
3. **US-7.1.4.3** — I want to place start and end markers on a NavMesh in the editor and see the
   funnel-generated waypoints displayed
   - **Acceptance:** I can iterate on level layout
4. **US-7.1.4.4** — I want NPCs to cut corners naturally instead of zigzagging between polygon
   centers
   - **Acceptance:** movement looks human-like
5. **US-7.1.4.5** — I want AI agents to move smoothly through narrow corridors without bouncing
   between walls
   - **Acceptance:** indoor navigation feels polished
6. **US-7.1.4.6** — I want NPCs to pass through doorways in a straight line rather than bouncing off
   door frames
   - **Acceptance:** entry and exit looks natural
7. **US-7.1.4.7** — I want to implement the funnel algorithm to convert polygon corridors into
   minimal waypoint sequences
   - **Acceptance:** path quality improves with low CPU cost
8. **US-7.1.4.8** — I want to guarantee that all funnel-generated waypoints lie within the NavMesh
   polygon boundaries
   - **Acceptance:** agents never path off-mesh
9. **US-7.1.4.9** — I want to feed funnel waypoints into the steering behavior system
   - **Acceptance:** path following and local avoidance work together seamlessly
10. **US-7.1.4.10** — I want to verify that the funnel algorithm produces fewer waypoints than the
    raw polygon count for reference corridors
    - **Acceptance:** paths are optimally short
11. **US-7.1.4.11** — I want to confirm that no funnel-generated waypoint lies outside the NavMesh
    boundaries
    - **Acceptance:** agents never attempt invalid movement
12. **US-7.1.4.12** — I want to measure funnel algorithm CPU time across varying corridor lengths
    - **Acceptance:** I can confirm it remains lightweight. ---

## F-7.1.5 — Path Smoothing

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.5.1  | designer (P-5)          | F-7.1.5  | R-7.1.5      |
| US-7.1.5.2  | designer (P-5)          | F-7.1.5  | R-7.1.5      |
| US-7.1.5.3  | player (P-23)           | F-7.1.5  | R-7.1.5      |
| US-7.1.5.4  | player (P-23)           | F-7.1.5  | R-7.1.5      |
| US-7.1.5.5  | player (P-23)           | F-7.1.5  | R-7.1.5      |
| US-7.1.5.6  | engine developer (P-26) | F-7.1.5  | R-7.1.5      |
| US-7.1.5.7  | engine developer (P-26) | F-7.1.5  | R-7.1.5      |
| US-7.1.5.8  | engine developer (P-26) | F-7.1.5  | R-7.1.5      |
| US-7.1.5.9  | engine tester (P-27)    | F-7.1.5  | R-7.1.5      |
| US-7.1.5.10 | engine tester (P-27)    | F-7.1.5  | R-7.1.5      |
| US-7.1.5.11 | engine tester (P-27)    | F-7.1.5  | R-7.1.5      |
| US-7.1.5.12 | engine tester (P-27)    | F-7.1.5  | R-7.1.5      |

1. **US-7.1.5.1** — I want to enable Catmull-Rom or Bezier smoothing for specific patrol routes
   - **Acceptance:** guard movements look natural and elegant
2. **US-7.1.5.2** — I want to select between linear, Catmull-Rom, and Bezier smoothing per agent
   archetype
   - **Acceptance:** different NPCs have movement styles matching their character
3. **US-7.1.5.3** — I want town patrol guards to walk in smooth curves around corners instead of
   making sharp right-angle turns
   - **Acceptance:** movement feels lifelike
4. **US-7.1.5.4** — I want pursuing enemies to run in natural arcs rather than jagged straight-line
   segments
   - **Acceptance:** chases feel cinematic
5. **US-7.1.5.5** — I want NPC movement to flow continuously between waypoints without stuttering or
   speed changes
   - **Acceptance:** path transitions are invisible
6. **US-7.1.5.6** — I want to post-process waypoint paths with NavMesh raycasts to remove redundant
   turns
   - **Acceptance:** paths are simplified before smoothing
7. **US-7.1.5.7** — I want to apply Catmull-Rom or cubic Bezier interpolation to waypoint paths
   - **Acceptance:** movement trajectories are smooth curves
8. **US-7.1.5.8** — I want to fall back to linear waypoint paths on mobile platforms
   - **Acceptance:** per-path CPU cost stays within the tighter mobile budget
9. **US-7.1.5.9** — I want to verify that smoothed curved paths do not leave the NavMesh boundary
   - **Acceptance:** agents never move through impassable geometry
10. **US-7.1.5.10** — I want to compare linear, Catmull-Rom, and Bezier paths on a reference route
    and capture screenshots
    - **Acceptance:** visual quality differences are documented
11. **US-7.1.5.11** — I want to measure the CPU overhead of path smoothing for 100+ agents
    simultaneously
    - **Acceptance:** I can confirm it fits within the navigation budget
12. **US-7.1.5.12** — I want to confirm that mobile platform builds use linear-only paths and never
    invoke Bezier interpolation
    - **Acceptance:** the platform gate works. ---

## F-7.1.6 — Dynamic Obstacle Carving

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.6.1  | designer (P-5)          | F-7.1.6  | R-7.1.6      |
| US-7.1.6.2  | designer (P-5)          | F-7.1.6  | R-7.1.6      |
| US-7.1.6.3  | designer (P-5)          | F-7.1.6  | R-7.1.6      |
| US-7.1.6.4  | player (P-23)           | F-7.1.6  | R-7.1.6      |
| US-7.1.6.5  | player (P-23)           | F-7.1.6  | R-7.1.6      |
| US-7.1.6.6  | player (P-23)           | F-7.1.6  | R-7.1.6      |
| US-7.1.6.7  | engine developer (P-26) | F-7.1.6  | R-7.1.6      |
| US-7.1.6.8  | engine developer (P-26) | F-7.1.6  | R-7.1.6      |
| US-7.1.6.9  | engine developer (P-26) | F-7.1.6  | R-7.1.6      |
| US-7.1.6.10 | engine tester (P-27)    | F-7.1.6  | R-7.1.6      |
| US-7.1.6.11 | engine tester (P-27)    | F-7.1.6  | R-7.1.6      |
| US-7.1.6.12 | engine tester (P-27)    | F-7.1.6  | R-7.1.6      |

1. **US-7.1.6.1** — I want to tag dynamic objects (barricades, vehicles) as NavMesh obstacles in the
   editor
   - **Acceptance:** agents automatically reroute around them
2. **US-7.1.6.2** — I want to place a barricade in an agent's path at runtime and see it reroute
   within seconds
   - **Acceptance:** I can verify dynamic carving responsiveness
3. **US-7.1.6.3** — I want to configure the carve shape and clearance margin per obstacle type
   - **Acceptance:** the blocked area matches the object's footprint
4. **US-7.1.6.4** — I want enemies to find an alternate route when a tree falls across their path
   - **Acceptance:** the world feels dynamically responsive
5. **US-7.1.6.5** — I want NPCs to walk around barricades I place instead of walking through them
   - **Acceptance:** my tactical placements affect AI movement
6. **US-7.1.6.6** — I want AI agents to adjust paths when a vehicle parks in their way
   - **Acceptance:** the world feels alive and reactive
7. **US-7.1.6.7** — I want to implement tile-local NavMesh re-carving that invalidates only affected
   polygons
   - **Acceptance:** carving cost is proportional to obstacle size
8. **US-7.1.6.8** — I want to trigger repath requests only for agents whose corridors intersect
   modified polygons
   - **Acceptance:** unaffected agents are not re-queried
9. **US-7.1.6.9** — I want to limit simultaneous carve operations to 4 per tick on mobile
   - **Acceptance:** carving does not exceed the platform CPU budget
10. **US-7.1.6.10** — I want to verify that agents cannot path through carved obstacle regions
    - **Acceptance:** dynamic obstacles are correctly blocking
11. **US-7.1.6.11** — I want to confirm that rapid obstacle placement does not cause a repath storm
    exceeding the CPU budget
    - **Acceptance:** frame rate remains stable
12. **US-7.1.6.12** — I want to verify that removing a dynamic obstacle restores the original
    NavMesh polygons
    - **Acceptance:** no ghost obstacles persist. ---

## F-7.1.7 — NavMesh Links (Off-Mesh Connections)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.7.1  | designer (P-5)          | F-7.1.7  | R-7.1.7      |
| US-7.1.7.2  | designer (P-5)          | F-7.1.7  | R-7.1.7      |
| US-7.1.7.3  | designer (P-5)          | F-7.1.7  | R-7.1.7      |
| US-7.1.7.4  | player (P-23)           | F-7.1.7  | R-7.1.7      |
| US-7.1.7.5  | player (P-23)           | F-7.1.7  | R-7.1.7      |
| US-7.1.7.6  | player (P-23)           | F-7.1.7  | R-7.1.7      |
| US-7.1.7.7  | engine developer (P-26) | F-7.1.7  | R-7.1.7      |
| US-7.1.7.8  | engine developer (P-26) | F-7.1.7  | R-7.1.7      |
| US-7.1.7.9  | engine developer (P-26) | F-7.1.7  | R-7.1.7      |
| US-7.1.7.10 | engine tester (P-27)    | F-7.1.7  | R-7.1.7      |
| US-7.1.7.11 | engine tester (P-27)    | F-7.1.7  | R-7.1.7      |
| US-7.1.7.12 | engine tester (P-27)    | F-7.1.7  | R-7.1.7      |

1. **US-7.1.7.1** — I want to place off-mesh links between disjoint NavMesh regions in the editor
   (jump points, ladder connections)
   - **Acceptance:** agents can traverse gaps
2. **US-7.1.7.2** — I want to assign animation tags (jump, climb, swim) to each off-mesh link
   - **Acceptance:** agents play the correct traversal animation
3. **US-7.1.7.3** — I want to set preconditions on off-mesh links (key required, ability unlocked)
   - **Acceptance:** only eligible agents use restricted connections
4. **US-7.1.7.4** — I want enemies to climb ladders and jump across gaps during pursuit
   - **Acceptance:** AI movement uses the full level geometry
5. **US-7.1.7.5** — I want NPCs to open doors along their path instead of stopping at closed
   doorways
   - **Acceptance:** they navigate the world as I do
6. **US-7.1.7.6** — I want guards to be unable to follow me through locked doors they cannot open
   - **Acceptance:** precondition gating feels fair and logical
7. **US-7.1.7.7** — I want to implement off-mesh connections carrying cost, animation tag, and
   precondition data
   - **Acceptance:** the planner can evaluate link feasibility
8. **US-7.1.7.8** — I want to incorporate off-mesh links into the A* polygon search with their
   associated costs
   - **Acceptance:** pathfinding considers multi-modal traversal
9. **US-7.1.7.9** — I want to support both bidirectional and one-way off-mesh links
   - **Acceptance:** jump-downs and one-way drops are modeled correctly
10. **US-7.1.7.10** — I want to verify that agents without the required ability cannot use
    precondition-gated links
    - **Acceptance:** link restrictions are enforced
11. **US-7.1.7.11** — I want to confirm that the correct traversal animation plays when an agent
    uses each link type
    - **Acceptance:** movement matches the animation tag
12. **US-7.1.7.12** — I want to verify that A* path costs correctly include off-mesh link traversal
    costs
    - **Acceptance:** the planner accurately compares link vs. walkable paths. ---

## F-7.1.8 — Incremental Tile Rebuild

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.8.1  | designer (P-5)          | F-7.1.8  | R-7.1.8      |
| US-7.1.8.2  | designer (P-5)          | F-7.1.8  | R-7.1.8      |
| US-7.1.8.3  | designer (P-5)          | F-7.1.8  | R-7.1.8      |
| US-7.1.8.4  | player (P-23)           | F-7.1.8  | R-7.1.8      |
| US-7.1.8.5  | player (P-23)           | F-7.1.8  | R-7.1.8      |
| US-7.1.8.6  | player (P-23)           | F-7.1.8  | R-7.1.8      |
| US-7.1.8.7  | engine developer (P-26) | F-7.1.8  | R-7.1.8      |
| US-7.1.8.8  | engine developer (P-26) | F-7.1.8  | R-7.1.8      |
| US-7.1.8.9  | engine developer (P-26) | F-7.1.8  | R-7.1.8      |
| US-7.1.8.10 | engine tester (P-27)    | F-7.1.8  | R-7.1.8      |
| US-7.1.8.11 | engine tester (P-27)    | F-7.1.8  | R-7.1.8      |
| US-7.1.8.12 | engine tester (P-27)    | F-7.1.8  | R-7.1.8      |

1. **US-7.1.8.1** — I want to deform terrain in the editor and see the NavMesh update for only the
   affected tiles
   - **Acceptance:** I can iterate on destructible environments quickly
2. **US-7.1.8.2** — I want to manually mark spatial regions as dirty to force a NavMesh rebuild
   - **Acceptance:** I can test incremental rebuild behavior in the editor
3. **US-7.1.8.3** — I want to set rebuild priority so tiles near active gameplay rebuild first
   - **Acceptance:** player-facing navigation updates quickly
4. **US-7.1.8.4** — I want AI to find new paths through areas opened by building destruction
   - **Acceptance:** the world responds dynamically to environmental changes
5. **US-7.1.8.5** — I want AI to reroute around rubble created by a collapsed structure
   - **Acceptance:** destruction meaningfully changes navigation
6. **US-7.1.8.6** — I want AI movement to remain smooth while NavMesh tiles rebuild in the
   background
   - **Acceptance:** I do not notice any navigation hitches
7. **US-7.1.8.7** — I want to use a `NavMeshDirtyRegion` ECS component to mark spatial extents for
   rebuild
   - **Acceptance:** the system processes them in priority order
8. **US-7.1.8.8** — I want to revoxelize only the affected tile and its immediate neighbors
   - **Acceptance:** rebuild cost scales with damage area, not world size
9. **US-7.1.8.9** — I want to cap concurrent tile rebuilds (1 on mobile, 4+ on desktop)
   - **Acceptance:** rebuild work fits within the platform's CPU budget
10. **US-7.1.8.10** — I want to verify that only the dirty tiles and their neighbors are rebuilt
    while all other tiles remain unchanged
    - **Acceptance:** rebuild scope is minimal
11. **US-7.1.8.11** — I want to compare incrementally rebuilt tiles against a full rebake and verify
    identical results
    - **Acceptance:** incremental and full builds are equivalent
12. **US-7.1.8.12** — I want to measure the time savings of incremental tile rebuild vs. full rebake
    on reference levels
    - **Acceptance:** the optimization is validated. ---

## F-7.1.9 — Background NavMesh Generation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.1.9.1  | designer (P-5)          | F-7.1.9  | R-7.1.9      |
| US-7.1.9.2  | designer (P-5)          | F-7.1.9  | R-7.1.9      |
| US-7.1.9.3  | designer (P-5)          | F-7.1.9  | R-7.1.9      |
| US-7.1.9.4  | player (P-23)           | F-7.1.9  | R-7.1.9      |
| US-7.1.9.5  | player (P-23)           | F-7.1.9  | R-7.1.9      |
| US-7.1.9.6  | player (P-23)           | F-7.1.9  | R-7.1.9      |
| US-7.1.9.7  | engine developer (P-26) | F-7.1.9  | R-7.1.9      |
| US-7.1.9.8  | engine developer (P-26) | F-7.1.9  | R-7.1.9      |
| US-7.1.9.9  | engine developer (P-26) | F-7.1.9  | R-7.1.9      |
| US-7.1.9.10 | engine tester (P-27)    | F-7.1.9  | R-7.1.9      |
| US-7.1.9.11 | engine tester (P-27)    | F-7.1.9  | R-7.1.9      |
| US-7.1.9.12 | engine tester (P-27)    | F-7.1.9  | R-7.1.9      |

1. **US-7.1.9.1** — I want the editor to remain fully responsive while NavMesh tiles generate in the
   background
   - **Acceptance:** I can continue working during long bakes
2. **US-7.1.9.2** — I want to see a progress indicator showing how many tiles remain in the
   background generation queue
   - **Acceptance:** I know when the bake will finish
3. **US-7.1.9.3** — I want to cancel an in-progress background NavMesh generation without corrupting
   the existing mesh
   - **Acceptance:** I can restart with new settings
4. **US-7.1.9.4** — I want gameplay to remain smooth while NavMesh tiles rebuild at runtime
   - **Acceptance:** world changes do not cause frame drops
5. **US-7.1.9.5** — I want AI agents to keep moving (using fallback navigation) while their tile is
   being rebuilt
   - **Acceptance:** NPCs never freeze in place
6. **US-7.1.9.6** — I want NPCs to transition seamlessly to newly built tiles without visible
   teleporting or path snapping
   - **Acceptance:** tile swaps are invisible
7. **US-7.1.9.7** — I want to run NavMesh generation on background worker threads via the job system
   - **Acceptance:** the main simulation tick is never blocked
8. **US-7.1.9.8** — I want to mark tiles under construction as `Pending` in the `NavMeshTileMap` ECS
   resource
   - **Acceptance:** agents receive fallback paths through pending areas
9. **US-7.1.9.9** — I want to swap completed tiles into the live NavMesh atomically at the next sync
   point
   - **Acceptance:** agents never read a partially built tile
10. **US-7.1.9.10** — I want to verify that NavMesh generation never blocks the main simulation
    thread
    - **Acceptance:** frame time is unaffected by background bakes
11. **US-7.1.9.11** — I want to verify that agents navigating through pending tiles use valid
    fallback movement
    - **Acceptance:** no agent gets stuck or moves through walls
12. **US-7.1.9.12** — I want to confirm that tile swaps are atomic and no agent ever reads partial
    tile data
    - **Acceptance:** data races are impossible. ---

## F-7.1.10 — Destruction-Triggered NavMesh Updates

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.1.10.1  | designer (P-5)          | F-7.1.10 | R-7.1.10     |
| US-7.1.10.2  | designer (P-5)          | F-7.1.10 | R-7.1.10     |
| US-7.1.10.3  | designer (P-5)          | F-7.1.10 | R-7.1.10     |
| US-7.1.10.4  | player (P-23)           | F-7.1.10 | R-7.1.10     |
| US-7.1.10.5  | player (P-23)           | F-7.1.10 | R-7.1.10     |
| US-7.1.10.6  | player (P-23)           | F-7.1.10 | R-7.1.10     |
| US-7.1.10.7  | engine developer (P-26) | F-7.1.10 | R-7.1.10     |
| US-7.1.10.8  | engine developer (P-26) | F-7.1.10 | R-7.1.10     |
| US-7.1.10.9  | engine developer (P-26) | F-7.1.10 | R-7.1.10     |
| US-7.1.10.10 | engine tester (P-27)    | F-7.1.10 | R-7.1.10     |
| US-7.1.10.11 | engine tester (P-27)    | F-7.1.10 | R-7.1.10     |
| US-7.1.10.12 | engine tester (P-27)    | F-7.1.10 | R-7.1.10     |

1. **US-7.1.10.1** — I want to destroy a building in the editor and see the NavMesh update to open
   new pathways
   - **Acceptance:** I can verify destruction integration works
2. **US-7.1.10.2** — I want rebuild priority to scale with the number of active agents whose paths
   cross the destroyed region
   - **Acceptance:** high-traffic areas update first
3. **US-7.1.10.3** — I want to verify that destroying a wall creates a usable NavMesh corridor
   through the breach
   - **Acceptance:** destruction has tactical pathfinding consequences
4. **US-7.1.10.4** — I want enemies to rush through a wall I just destroyed, finding new paths
   through the breach
   - **Acceptance:** destruction has tactical consequences
5. **US-7.1.10.5** — I want AI to treat rubble from collapsed structures as obstacles, routing
   around debris
   - **Acceptance:** destruction creates new navigation challenges
6. **US-7.1.10.6** — I want AI near a destruction event to repath within 1-2 seconds
   - **Acceptance:** their response feels immediate and aware
7. **US-7.1.10.7** — I want the destruction system to emit `NavMeshInvalidation` ECS events with
   affected bounding regions
   - **Acceptance:** the rebuild system picks them up
8. **US-7.1.10.8** — I want the `NavMeshRebuildSystem` to enqueue tile rebuilds with priority
   proportional to affected agent count
   - **Acceptance:** critical areas rebuild first
9. **US-7.1.10.9** — I want to handle destruction events that affect tiles already in the rebuild
   queue by merging dirty regions
   - **Acceptance:** redundant rebuilds are avoided
10. **US-7.1.10.10** — I want to verify that destroying a wall creates a passable NavMesh corridor
    through the breach
    - **Acceptance:** agents can path through openings
11. **US-7.1.10.11** — I want to verify that rubble from destruction is represented as NavMesh
    obstacles
    - **Acceptance:** agents cannot walk through debris piles
12. **US-7.1.10.12** — I want to confirm that mobile defers or skips rebuilds for distant
    destruction events
    - **Acceptance:** mobile CPU budget is respected. ---

## F-7.1.11 — Player-Built Structure Integration

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.1.11.1  | designer (P-5)          | F-7.1.11 | R-7.1.11     |
| US-7.1.11.2  | designer (P-5)          | F-7.1.11 | R-7.1.11     |
| US-7.1.11.3  | designer (P-5)          | F-7.1.11 | R-7.1.11     |
| US-7.1.11.4  | player (P-23)           | F-7.1.11 | R-7.1.11     |
| US-7.1.11.5  | player (P-23)           | F-7.1.11 | R-7.1.11     |
| US-7.1.11.6  | player (P-23)           | F-7.1.11 | R-7.1.11     |
| US-7.1.11.7  | engine developer (P-26) | F-7.1.11 | R-7.1.11     |
| US-7.1.11.8  | engine developer (P-26) | F-7.1.11 | R-7.1.11     |
| US-7.1.11.9  | engine developer (P-26) | F-7.1.11 | R-7.1.11     |
| US-7.1.11.10 | engine tester (P-27)    | F-7.1.11 | R-7.1.11     |
| US-7.1.11.11 | engine tester (P-27)    | F-7.1.11 | R-7.1.11     |
| US-7.1.11.12 | engine tester (P-27)    | F-7.1.11 | R-7.1.11     |

1. **US-7.1.11.1** — I want to place a player-buildable structure in the editor and see the NavMesh
   update to reflect the new obstacle
   - **Acceptance:** I can verify building integration
2. **US-7.1.11.2** — I want to set area type overrides on player structures (wall, ramp, platform)
   - **Acceptance:** the NavMesh correctly classifies the structure
3. **US-7.1.11.3** — I want stairs and ladders on player structures to auto-generate off-mesh links
   - **Acceptance:** agents can traverse multi-level player buildings
4. **US-7.1.11.4** — I want enemies to navigate around the walls of my fort instead of walking
   through them
   - **Acceptance:** my building efforts affect AI pathing
5. **US-7.1.11.5** — I want AI allies to walk up ramps I build to reach elevated platforms
   - **Acceptance:** my constructions are usable by NPCs
6. **US-7.1.11.6** — I want AI to find new routes when I remove a wall section from my base
   - **Acceptance:** the NavMesh updates dynamically with my edits
7. **US-7.1.11.7** — I want to register player structures as NavMesh obstacles via a
   `NavMeshObstacle` ECS component with shape data
   - **Acceptance:** tiles rebuild on placement
8. **US-7.1.11.8** — I want to auto-generate off-mesh links for stairs and ladder attachment points
   on player structures
   - **Acceptance:** vertical traversal works automatically
9. **US-7.1.11.9** — I want to queue incremental tile rebuilds when structures are placed or removed
   - **Acceptance:** the NavMesh reflects the latest player construction state
10. **US-7.1.11.10** — I want to verify that placed structures marked as impassable walls correctly
    block agent pathfinding
    - **Acceptance:** walls function as barriers
11. **US-7.1.11.11** — I want to verify that agents can traverse auto-generated links on
    player-built ramps and ladders
    - **Acceptance:** vertical movement works end-to-end
12. **US-7.1.11.12** — I want to verify that removing a player structure restores the NavMesh to its
    pre-placement state
    - **Acceptance:** no ghost obstacles remain. ---

## F-7.1.12 — Multi-Size Agent NavMeshes

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.1.12.1  | designer (P-5)          | F-7.1.12 | R-7.1.12     |
| US-7.1.12.2  | designer (P-5)          | F-7.1.12 | R-7.1.12     |
| US-7.1.12.3  | designer (P-5)          | F-7.1.12 | R-7.1.12     |
| US-7.1.12.4  | player (P-23)           | F-7.1.12 | R-7.1.12     |
| US-7.1.12.5  | player (P-23)           | F-7.1.12 | R-7.1.12     |
| US-7.1.12.6  | player (P-23)           | F-7.1.12 | R-7.1.12     |
| US-7.1.12.7  | engine developer (P-26) | F-7.1.12 | R-7.1.12     |
| US-7.1.12.8  | engine developer (P-26) | F-7.1.12 | R-7.1.12     |
| US-7.1.12.9  | engine developer (P-26) | F-7.1.12 | R-7.1.12     |
| US-7.1.12.10 | engine tester (P-27)    | F-7.1.12 | R-7.1.12     |
| US-7.1.12.11 | engine tester (P-27)    | F-7.1.12 | R-7.1.12     |
| US-7.1.12.12 | engine tester (P-27)    | F-7.1.12 | R-7.1.12     |

1. **US-7.1.12.1** — I want to define multiple agent size classes (humanoid, mounted, siege golem)
   with radius, height, and slope parameters
   - **Acceptance:** each gets its own NavMesh
2. **US-7.1.12.2** — I want to assign each agent entity to a NavMesh layer via a `NavMeshAgent` ECS
   component
   - **Acceptance:** it paths on the mesh matching its size
3. **US-7.1.12.3** — I want to toggle visibility of individual NavMesh layers in the editor overlay
   - **Acceptance:** I can inspect each size class independently
4. **US-7.1.12.4** — I want large siege golems to take wide routes and avoid narrow alleys
   - **Acceptance:** their pathing matches their visible size
5. **US-7.1.12.5** — I want mounted riders to path along roads and open terrain rather than through
   tight doorways
   - **Acceptance:** mount navigation feels correct
6. **US-7.1.12.6** — I want small NPCs to take shortcuts through alleys that large NPCs cannot
   - **Acceptance:** size-based navigation differences are visible
7. **US-7.1.12.7** — I want to maintain multiple NavMesh layers sharing the same spatial tile grid
   - **Acceptance:** streaming loads all layers for a region together
8. **US-7.1.12.8** — I want to define `NavMeshAgentConfig` structs per size class with radius,
   height, step-climb, and max slope
   - **Acceptance:** each layer is generated correctly
9. **US-7.1.12.9** — I want to limit NavMesh layers to 2 on mobile (humanoid + large)
   - **Acceptance:** memory and streaming bandwidth stay within mobile constraints
10. **US-7.1.12.10** — I want to verify that each NavMesh layer produces correct walkable surfaces
    for its configured agent size
    - **Acceptance:** no layer has missing or extra coverage
11. **US-7.1.12.11** — I want to verify that each agent queries only its assigned NavMesh layer and
    never paths on a wrong-sized mesh
    - **Acceptance:** layer assignment is enforced
12. **US-7.1.12.12** — I want to measure the total memory consumed by multiple NavMesh layers
    - **Acceptance:** multi-layer overhead is within the allocated budget. ---

## F-7.1.13 — Dynamic Area Cost Modification

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.1.13.1  | designer (P-5)          | F-7.1.13 | R-7.1.13     |
| US-7.1.13.2  | designer (P-5)          | F-7.1.13 | R-7.1.13     |
| US-7.1.13.3  | designer (P-5)          | F-7.1.13 | R-7.1.13     |
| US-7.1.13.4  | player (P-23)           | F-7.1.13 | R-7.1.13     |
| US-7.1.13.5  | player (P-23)           | F-7.1.13 | R-7.1.13     |
| US-7.1.13.6  | player (P-23)           | F-7.1.13 | R-7.1.13     |
| US-7.1.13.7  | engine developer (P-26) | F-7.1.13 | R-7.1.13     |
| US-7.1.13.8  | engine developer (P-26) | F-7.1.13 | R-7.1.13     |
| US-7.1.13.9  | engine developer (P-26) | F-7.1.13 | R-7.1.13     |
| US-7.1.13.10 | engine tester (P-27)    | F-7.1.13 | R-7.1.13     |
| US-7.1.13.11 | engine tester (P-27)    | F-7.1.13 | R-7.1.13     |
| US-7.1.13.12 | engine tester (P-27)    | F-7.1.13 | R-7.1.13     |

1. **US-7.1.13.1** — I want to modify NavMesh polygon area costs at runtime in the editor (e.g.,
   flood a zone to raise water cost)
   - **Acceptance:** I can test dynamic routing
2. **US-7.1.13.2** — I want to set per-faction area cost overrides so friendly territory is cheaper
   for allies
   - **Acceptance:** faction-aware routing is configurable
3. **US-7.1.13.3** — I want to visualize a cost heatmap overlay on the NavMesh in the editor
   - **Acceptance:** I can see how area costs influence route selection
4. **US-7.1.13.4** — I want NPCs to prefer dry paths when a zone floods, avoiding the water unless
   no alternative exists
   - **Acceptance:** AI reacts to environmental changes
5. **US-7.1.13.5** — I want civilian NPCs to detour around active battlefields with danger area
   weighting
   - **Acceptance:** non-combatants flee to safety
6. **US-7.1.13.6** — I want allied troops to prefer roads in friendly territory while enemies take
   longer concealed routes
   - **Acceptance:** faction strategy is visible
7. **US-7.1.13.7** — I want to store runtime area cost modifications in a `NavMeshAreaCosts` ECS
   resource applied during A* expansion
   - **Acceptance:** costs change without rebuilding geometry
8. **US-7.1.13.8** — I want to support per-agent cost overrides for faction-specific routing
   - **Acceptance:** different factions evaluate the same area differently
9. **US-7.1.13.9** — I want to ensure area cost modifications are pure data updates with no geometry
   rebuild
   - **Acceptance:** cost changes are instantaneous and lightweight
10. **US-7.1.13.10** — I want to verify that increasing an area's cost causes agents to choose
    alternate routes when available
    - **Acceptance:** cost modifications affect pathfinding
11. **US-7.1.13.11** — I want to place two agents from different factions and verify they take
    different routes based on faction cost overrides
    - **Acceptance:** faction-aware routing works
12. **US-7.1.13.12** — I want to confirm that modifying area costs does not trigger any NavMesh tile
    rebuild
    - **Acceptance:** cost changes remain a zero-rebuild operation. ---

## F-7.1.14 — Hierarchical Pathfinding (HPA*)

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.1.14.1  | designer (P-5)          | F-7.1.14 | R-7.1.14     |
| US-7.1.14.2  | designer (P-5)          | F-7.1.14 | R-7.1.14     |
| US-7.1.14.3  | designer (P-5)          | F-7.1.14 | R-7.1.14     |
| US-7.1.14.4  | player (P-23)           | F-7.1.14 | R-7.1.14     |
| US-7.1.14.5  | player (P-23)           | F-7.1.14 | R-7.1.14     |
| US-7.1.14.6  | player (P-23)           | F-7.1.14 | R-7.1.14     |
| US-7.1.14.7  | engine developer (P-26) | F-7.1.14 | R-7.1.14     |
| US-7.1.14.8  | engine developer (P-26) | F-7.1.14 | R-7.1.14     |
| US-7.1.14.9  | engine developer (P-26) | F-7.1.14 | R-7.1.14     |
| US-7.1.14.10 | engine tester (P-27)    | F-7.1.14 | R-7.1.14     |
| US-7.1.14.11 | engine tester (P-27)    | F-7.1.14 | R-7.1.14     |
| US-7.1.14.12 | engine tester (P-27)    | F-7.1.14 | R-7.1.14     |

1. **US-7.1.14.1** — I want to enable hierarchical pathfinding for paths that cross multiple NavMesh
   tiles
   - **Acceptance:** long-distance NPC travel is efficient
2. **US-7.1.14.2** — I want to visualize the coarse hierarchical navigation graph in the editor
   - **Acceptance:** I can inspect cross-tile connectivity
3. **US-7.1.14.3** — I want to command an NPC to travel across the entire world map and verify it
   arrives
   - **Acceptance:** long-distance pathfinding is reliable
4. **US-7.1.14.4** — I want NPCs to begin long-distance journeys immediately without visible
   planning delay
   - **Acceptance:** travel commands feel responsive
5. **US-7.1.14.5** — I want trade caravans to navigate across multiple regions on efficient routes
   - **Acceptance:** cross-world travel looks intentional and planned
6. **US-7.1.14.6** — I want my AI companion to follow me across the entire map without losing its
   way
   - **Acceptance:** long adventures feel seamless
7. **US-7.1.14.7** — I want to build a coarse navigation graph from NavMesh tile connectivity
   - **Acceptance:** long-distance queries are O(1) per tile
8. **US-7.1.14.8** — I want to refine hierarchical paths to full NavMesh resolution only for the
   tiles the agent is currently traversing
   - **Acceptance:** detail is computed just-in-time
9. **US-7.1.14.9** — I want pathfinding cost to remain bounded regardless of world size
   - **Acceptance:** thousands of server-side NPCs can plan cross-continent paths
10. **US-7.1.14.10** — I want to verify that HPA* produces valid navigable paths between any two
    reachable points on the world map
    - **Acceptance:** no routes are broken
11. **US-7.1.14.11** — I want to compare HPA* paths against full A* paths on reference scenarios and
    measure cost deviation
    - **Acceptance:** hierarchical approximation is acceptable
12. **US-7.1.14.12** — I want to benchmark HPA* with 1000+ concurrent long-distance queries
    - **Acceptance:** server-side NPC travel scales to MMO population levels. ---

## F-7.1.15 — NavMesh Runtime Visualization

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.1.15.1  | designer (P-5)          | F-7.1.15 | R-7.1.15     |
| US-7.1.15.2  | designer (P-5)          | F-7.1.15 | R-7.1.15     |
| US-7.1.15.3  | designer (P-5)          | F-7.1.15 | R-7.1.15     |
| US-7.1.15.4  | designer (P-5)          | F-7.1.15 | R-7.1.15     |
| US-7.1.15.5  | player (P-23)           | F-7.1.15 | R-7.1.15     |
| US-7.1.15.6  | engine developer (P-26) | F-7.1.15 | R-7.1.15     |
| US-7.1.15.7  | engine developer (P-26) | F-7.1.15 | R-7.1.15     |
| US-7.1.15.8  | engine developer (P-26) | F-7.1.15 | R-7.1.15     |
| US-7.1.15.9  | designer (P-5)          | F-7.1.15 | R-7.1.15     |
| US-7.1.15.10 | engine tester (P-27)    | F-7.1.15 | R-7.1.15     |
| US-7.1.15.11 | engine tester (P-27)    | F-7.1.15 | R-7.1.15     |
| US-7.1.15.12 | engine tester (P-27)    | F-7.1.15 | R-7.1.15     |

1. **US-7.1.15.1** — I want to toggle a debug overlay showing NavMesh polygons, tile boundaries, and
   area types in the editor
   - **Acceptance:** I can inspect navigation data visually
2. **US-7.1.15.2** — I want different area types to be color-coded in the debug overlay (roads
   green, swamps brown, lava red)
   - **Acceptance:** I can identify zones at a glance
3. **US-7.1.15.3** — I want to see agent path trails drawn as colored lines on the NavMesh overlay
   - **Acceptance:** I can diagnose why agents choose specific routes
4. **US-7.1.15.4** — I want off-mesh links to appear as labeled arcs in the debug overlay
   - **Acceptance:** I can verify link placement and connectivity
5. **US-7.1.15.5** — I want no NavMesh debug overlays to appear in the shipping game
   - **Acceptance:** the visual experience is clean and immersive
6. **US-7.1.15.6** — I want to control visualization through `NavMeshDebug` ECS components
   - **Acceptance:** debug rendering is opt-in and configurable per entity or region
7. **US-7.1.15.7** — I want to integrate NavMesh visualization with the editor gizmo system
   (F-15.1.4)
   - **Acceptance:** overlays render within the existing debug infrastructure
8. **US-7.1.15.8** — I want to strip all NavMesh debug visualization code from shipping builds
   - **Acceptance:** release binaries have no debug overhead
9. **US-7.1.15.9** — I want tiles queued for rebuild to be highlighted in the overlay
   - **Acceptance:** I can see which regions are temporarily stale
10. **US-7.1.15.10** — I want to verify that the debug overlay accurately represents the underlying
    NavMesh data
    - **Acceptance:** visualization is trustworthy for debugging
11. **US-7.1.15.11** — I want to confirm that shipping builds contain no NavMesh debug rendering
    code or data
    - **Acceptance:** release builds are clean
12. **US-7.1.15.12** — I want to measure overlay rendering performance with the entire NavMesh
    visible
    - **Acceptance:** debug visualization does not tank editor frame rate
