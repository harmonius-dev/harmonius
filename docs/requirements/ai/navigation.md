# R-7.1 -- Navigation Requirements

## NavMesh Generation

1. **R-7.1.1** -- The engine **SHALL** generate polygonal navigation meshes from world geometry via
   heightfield voxelization, watershed partitioning, and contour tracing, with configurable agent
   radius, height, step climb, and slope limits per agent archetype.
   - **Rationale:** Automated NavMesh generation from geometry eliminates manual placement and
     ensures meshes match the world across dynamic terrain changes.
   - **Verification:** Generate a NavMesh for a scene with stairs, ramps, and walls. Verify all
     walkable surfaces within the slope limit are covered. Configure two agent archetypes and verify
     the large-radius mesh excludes narrow passages the small-radius mesh includes.

2. **R-7.1.2** -- The engine **SHALL** divide the NavMesh into fixed-size tiles aligned to the world
   streaming grid, loading and unloading tiles asynchronously with seamless cross-tile pathfinding.
   - **Rationale:** Open-world navigation requires bounded memory usage; streaming keeps the active
     NavMesh within budget regardless of total world size.
   - **Verification:** Load a world with 100+ tiles. Verify only tiles within the streaming window
     are resident. Request a path crossing 5 tile boundaries and verify it is valid and continuous.

## Pathfinding

1. **R-7.1.3** -- The engine **SHALL** perform A* search over NavMesh polygons with configurable
   per-area-type cost weights and a per-tick CPU budget that batches queries across frames.
   - **Rationale:** Cost-weighted pathfinding enables contextual route preferences while batching
     prevents frame-time spikes.
   - **Verification:** Configure lava with infinite cost and verify no path crosses it. Configure
     roads with cost 0.5 and verify paths prefer roads. Issue 200 simultaneous queries and verify
     frame time stays within budget.

2. **R-7.1.4** -- The engine **SHALL** convert NavMesh polygon corridors into minimal waypoint
   sequences using the funnel algorithm, producing paths with no more than turn_count + 2 waypoints.
   - **Rationale:** Raw polygon corridors produce inefficient zig-zag movement; the funnel algorithm
     yields direct, natural paths.
   - **Verification:** Generate a corridor through an L-shaped hallway and verify the funnel
     produces exactly 3 waypoints. Verify waypoint count never exceeds turn_count + 2 across 100
     randomized paths.

3. **R-7.1.5** -- The engine **SHALL** post-process waypoint paths with raycast validation and
   optional Catmull-Rom or Bezier interpolation, where all interpolated points lie on valid NavMesh
   polygons.
   - **Rationale:** Smoothed paths eliminate robotic movement and improve visual quality for patrol
     routes.
   - **Verification:** Smooth a 10-waypoint path with Catmull-Rom interpolation. Sample 100 points
     along the curve and verify each lies on a valid NavMesh polygon.

## Dynamic Obstacles and Links

1. **R-7.1.6** -- The engine **SHALL** mark NavMesh regions as temporarily blocked when dynamic
   obstacles appear, using tile-local re-carving that invalidates only affected polygons.
   - **Rationale:** Dynamic obstacles must update navigation without full mesh rebuild; localized
     carving bounds the cost.
   - **Verification:** Place a barricade and verify the affected polygons are removed. Verify agents
     whose corridors intersect the modified area receive repath requests. Remove the barricade and
     verify the polygons are restored.

2. **R-7.1.7** -- The engine **SHALL** support off-mesh connections between disjoint NavMesh regions
   with cost, animation tag, and optional precondition per link.
   - **Rationale:** Jumps, ladders, doors, and swimming transitions require explicit connections
     between otherwise disconnected mesh regions.
   - **Verification:** Create a link between two platforms. Verify the pathfinder includes the link
     in plans. Add a precondition and verify agents without the capability exclude the link.

## Runtime NavMesh Rebuilding

1. **R-7.1.8** -- The engine **SHALL** rebuild individual NavMesh tiles at runtime when world
   geometry changes, revoxelizing only affected tiles and their immediate neighbors.
   - **Rationale:** Terrain deformation, building destruction, and player structures require runtime
     mesh updates without full rebuild cost.
   - **Verification:** Deform terrain in one tile and verify only that tile and its neighbors
     rebuild. Verify unaffected tiles remain untouched and agents continue navigating.

2. **R-7.1.9** -- The engine **SHALL** run NavMesh generation and rebuild on background worker
   threads, never blocking the main simulation tick.
   - **Rationale:** NavMesh generation is CPU-intensive and must not cause frame-time spikes.
   - **Verification:** Trigger a tile rebuild and verify the main tick frame time does not increase.
     Verify agents with corridors through pending tiles receive a fallback path.

3. **R-7.1.10** -- The engine **SHALL** trigger incremental NavMesh rebuilds when destructible
   entities fracture, with rebuild priority proportional to the number of affected agents.
   - **Rationale:** Destruction opens new pathways and creates new obstacles; navigation must
     reflect these changes automatically.
   - **Verification:** Destroy a wall and verify the affected tiles rebuild. Verify agents discover
     the new pathway within the rebuild latency.

4. **R-7.1.11** -- The engine **SHALL** register player-placed structures as NavMesh obstacles with
   area type overrides and auto-generate NavMesh links for stairs and ladders.
   - **Rationale:** Player-built structures must be navigable by AI without manual link placement.
   - **Verification:** Place a two-story structure with stairs. Verify the tiles rebuild with the
     structure as an obstacle. Verify auto-generated links allow agents to traverse the stairs.

## Multi-Agent Navigation

1. **R-7.1.12** -- The engine **SHALL** maintain multiple NavMesh layers for different agent size
   classes, sharing the same spatial grid for streaming.
   - **Rationale:** Humanoids, mounts, and large creatures require separate meshes matching their
     dimensions.
   - **Verification:** Generate meshes for radius 0.3 m and 1.0 m agents. Verify the large agent
     cannot enter a narrow passage the small agent can. Verify streaming loads both layers for a
     region.

2. **R-7.1.13** -- The engine **SHALL** support runtime area cost modification without geometry
   rebuild, including per-agent cost overrides.
   - **Rationale:** Dynamic cost changes like flooded zones and faction bonuses must affect
     pathfinding without expensive mesh operations.
   - **Verification:** Raise the cost of a zone and verify agents prefer alternate routes. Apply a
     per-agent faction override and verify routing differs between factions.

## Long-Distance Pathfinding

1. **R-7.1.14** -- The engine **SHALL** provide hierarchical pathfinding using a coarse
   tile-connectivity graph, keeping path cost bounded regardless of world size.
   - **Rationale:** Thousands of server-side NPCs with cross-continent goals require O(1)-per-tile
     long-distance planning.
   - **Verification:** Plan a path spanning 50+ tiles using hierarchical search and verify it
     completes within the per-tick budget. Verify the path refines to full NavMesh resolution only
     for the agent's current tile.

## Debugging and Visualization

1. **R-7.1.15** -- The engine **SHALL** render NavMesh polygons, tile boundaries, area types,
   obstacle carve regions, off-mesh links, and pending rebuild zones as editor debug overlays,
   stripped from shipping builds.
   - **Rationale:** Designers need visual feedback to diagnose navigation failures in complex
     geometry.
   - **Verification:** Enable the debug overlay and verify all listed elements are visible and
     color-coded. Verify the overlay is absent from a shipping build.

## NavMesh Surface Raycasting

1. **R-7.1.16** -- The engine **SHALL** support raycasting along the NavMesh surface within tiles
   for path validation and redundant turn removal.
   - **Rationale:** Surface raycasts enable path post-processing to eliminate unnecessary waypoints
     and validate line-of-sight between path segments on the mesh.
   - **Verification:** Cast a ray between two points on the same tile and verify the hit point lies
     on a valid NavMesh polygon. Use surface raycasts during path smoothing and verify redundant
     turns are removed.

## Stale Tile Fallback

1. **R-7.1.17** -- The engine **SHALL** serve stale NavMesh tile data as fallback during background
   tile rebuilds so agents continue navigating with degraded but functional paths.
   - **Rationale:** Agents must not freeze during tile rebuilds; stale data provides continuity
     until the new tile is ready.
   - **Verification:** Trigger a tile rebuild and verify agents with corridors through the pending
     tile continue navigating on stale data. Verify the stale tile is replaced atomically when the
     rebuild completes.

## Per-Query Polygon Filters

1. **R-7.1.18** -- The engine **SHALL** support per-query polygon flag filters (include and exclude
   bitmasks) enabling agent-specific traversal restrictions.
   - **Rationale:** Flag-based filtering lets different agent types exclude specific polygon classes
     (water, lava, faction zones) without separate NavMesh layers.
   - **Verification:** Mark polygons with a water flag. Query with water excluded and verify the
     path avoids water polygons. Query with water included and verify the path crosses them.
