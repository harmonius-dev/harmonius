# R-7.1 -- Navigation Requirements

## R-7.1.1 Recast-Style NavMesh Generation

The engine **SHALL** voxelize static world geometry into a heightfield and build a compact polygonal
navigation mesh via watershed partitioning and contour tracing, accepting configurable agent radius,
height, step climb, and slope limits that produce distinct meshes per entity archetype.

- **Derived from:** [F-7.1.1](../../features/ai/navigation.md)
- **Rationale:** Different entity sizes (humanoid, mount, large creature) need tailored meshes to
  navigate geometry appropriate to their physical dimensions.
- **Verification:** Generate NavMeshes for three agent archetypes with different radii (0.3 m,
  0.6 m, 1.5 m) on a test scene with narrow doorways and steep slopes. Verify that the smallest
  agent's mesh includes narrow passages excluded from the largest agent's mesh, and that polygons
  respect the configured slope limit within 1 degree tolerance.

## R-7.1.2 NavMesh Tiling and Streaming

The engine **SHALL** divide the navigation mesh into fixed-size tiles aligned with the world
streaming grid and load/unload tiles asynchronously as the simulation window moves, without
requiring the full mesh to reside in memory.

- **Derived from:** [F-7.1.2](../../features/ai/navigation.md)
- **Rationale:** Open-world navigation requires streaming to stay within server memory budgets
  while providing seamless cross-tile pathfinding.
- **Verification:** Configure a world with 100 NavMesh tiles. Stream in a 5x5 window and verify
  that only 25 tiles are loaded in memory. Move the window by one tile and confirm the leading
  edge tiles load and trailing edge tiles unload within one streaming tick.

## R-7.1.3 A* Pathfinding on NavMesh

The engine **SHALL** perform hierarchical A* search over NavMesh polygons with configurable
heuristics and per-area-type cost weights, batching queries across frames to remain within a
configurable per-tick CPU budget.

- **Derived from:** [F-7.1.3](../../features/ai/navigation.md)
- **Rationale:** Area-type weighting lets AI prefer contextually appropriate routes (roads over
  swamps), and batching prevents pathfinding from monopolizing CPU time.
- **Verification:** Issue 500 path queries on a NavMesh with road (cost 1.0), swamp (cost 5.0),
  and lava (cost infinity) areas. Verify all paths prefer roads over swamps and never cross lava.
  Confirm per-tick CPU time stays within the configured budget by measuring wall-clock time.

## R-7.1.4 String Pulling (Funnel Algorithm)

The engine **SHALL** convert a corridor of NavMesh polygons into a minimal waypoint sequence using
the funnel algorithm, producing corner-hugging paths with no unnecessary intermediate waypoints.

- **Derived from:** [F-7.1.4](../../features/ai/navigation.md)
- **Rationale:** The funnel algorithm eliminates redundant waypoints, producing tighter paths
  that avoid unnecessary detours and reduce steering input complexity.
- **Verification:** Given a corridor of 20 polygons with two sharp turns, verify the funnel
  algorithm outputs exactly the minimal set of waypoints (no more than turn count + 2). Confirm
  each waypoint lies on a polygon boundary or start/end position.

## R-7.1.5 Path Smoothing

The engine **SHALL** post-process waypoint paths with NavMesh raycasts to remove redundant turns
and optionally apply Catmull-Rom or cubic Bezier interpolation for curved trajectories.

- **Derived from:** [F-7.1.5](../../features/ai/navigation.md)
- **Rationale:** Smoothed paths produce natural-looking movement for patrol routes and aesthetic
  scenarios where sharp angular turns look artificial.
- **Verification:** Smooth a 10-waypoint path and verify the output has fewer waypoints than the
  input. Confirm all interpolated positions lie on valid NavMesh polygons by raycasting each
  sample point. Visually compare smoothed vs. unsmoothed paths and verify no path segment exits
  the NavMesh.

## R-7.1.6 Dynamic Obstacle Carving

The engine **SHALL** mark NavMesh regions as temporarily blocked when dynamic obstacles appear or
move, using tile-local re-carving that invalidates only affected polygons and triggers localized
repath requests for agents whose corridors intersect the modified area.

- **Derived from:** [F-7.1.6](../../features/ai/navigation.md)
- **Rationale:** Dynamic obstacles (barricades, vehicles) must immediately affect navigation
  without a full mesh rebuild to maintain responsive AI movement.
- **Verification:** Place a dynamic obstacle blocking an agent's active corridor. Verify the
  affected polygons are invalidated within one tick, the agent receives a repath request, and
  unaffected tiles remain unchanged. Remove the obstacle and confirm the original polygons are
  restored.

## R-7.1.7 NavMesh Links (Off-Mesh Connections)

The engine **SHALL** support traversal connections between disjoint NavMesh regions, each carrying
a cost, animation tag, and optional precondition, so the planner can evaluate feasibility of
actions like jumping, climbing, or swimming.

- **Derived from:** [F-7.1.7](../../features/ai/navigation.md)
- **Rationale:** Real-world navigation includes non-walking transitions (ladders, gaps, doors)
  that require explicit links between disconnected mesh regions.
- **Verification:** Create two NavMesh regions separated by a gap with a jump link (cost 5.0,
  precondition: ability_jump = true). Verify an agent with the ability uses the link and an
  agent without it finds an alternative path. Confirm the link cost is included in A* total
  path cost.

## R-7.1.8 Incremental Tile Rebuild

The engine **SHALL** rebuild individual NavMesh tiles at runtime when world geometry changes,
revoxelizing only the affected tile and its immediate neighbors while leaving unaffected tiles
untouched, driven by `NavMeshDirtyRegion` ECS components processed in priority order.

- **Derived from:** [F-7.1.8](../../features/ai/navigation.md)
- **Rationale:** Full mesh rebuilds are too expensive for runtime geometry changes; incremental
  rebuilds keep navigation current with bounded cost.
- **Verification:** Modify geometry in a single tile region. Verify only that tile and its
  direct neighbors are rebuilt (measure rebuild count). Confirm agents on distant tiles
  experience no navigation disruption. Verify `NavMeshDirtyRegion` components are consumed
  after processing.

## R-7.1.9 Background NavMesh Generation

The engine **SHALL** run NavMesh tile generation and rebuild on background worker threads via the
job system, never blocking the main simulation tick, with pending tiles marked as `Pending` in the
`NavMeshTileMap` ECS resource and completed tiles swapped in atomically at the next sync point.

- **Derived from:** [F-7.1.9](../../features/ai/navigation.md)
- **Rationale:** Blocking the main simulation for mesh generation would cause frame-time spikes;
  background generation keeps the tick rate stable.
- **Verification:** Trigger a tile rebuild and measure main-thread frame time during generation.
  Verify frame time does not exceed the baseline by more than 5%. Confirm agents in the pending
  tile receive fallback movement (straight-line with collision avoidance) until the new tile is
  swapped in.

## R-7.1.10 Destruction-Triggered NavMesh Updates

The engine **SHALL** observe `NavMeshInvalidation` ECS events emitted by the destruction system
when a `Destructible` entity fractures, and enqueue incremental tile rebuilds for the affected
bounding region with priority scaled by the number of active agents whose paths intersect the
region.

- **Derived from:** [F-7.1.10](../../features/ai/navigation.md)
- **Rationale:** Collapsed buildings open new pathways and rubble creates obstacles; navigation
  must reflect destruction outcomes for believable AI behavior.
- **Verification:** Destroy a building entity and verify a `NavMeshInvalidation` event is
  emitted. Confirm the rebuild system enqueues the affected tiles. Place 10 agents with paths
  through the region and 2 agents elsewhere; verify the affected-region rebuild has higher
  priority. Confirm agents repath through newly opened areas after rebuild completes.

## R-7.1.11 Player-Built Structure Integration

The engine **SHALL** register player-placed structures as NavMesh obstacles via a
`NavMeshObstacle` ECS component with shape data and area type override, queuing affected tiles
for incremental rebuild when structures are placed or removed, and auto-generating NavMesh links
for stairs and ladders.

- **Derived from:** [F-7.1.11](../../features/ai/navigation.md)
- **Rationale:** Player-built structures dynamically alter the navigable world; AI must route
  around walls and over ramps placed by players.
- **Verification:** Place a player wall structure and verify AI agents repath around it after
  tile rebuild. Place a ramp structure and verify a NavMesh link is auto-generated connecting
  the ground to the elevated platform. Remove the structure and confirm the NavMesh reverts to
  its prior state.

## R-7.1.12 Multi-Size Agent NavMeshes

The engine **SHALL** maintain multiple NavMesh layers for different agent size classes, each
defined by a `NavMeshAgentConfig` (radius, height, step climb, max slope), sharing the same
spatial tile grid so streaming loads all layers for a region together, with agent entities
referencing their layer via a `NavMeshAgent` ECS component.

- **Derived from:** [F-7.1.12](../../features/ai/navigation.md)
- **Rationale:** Different agent sizes require separate mesh layers to ensure each agent
  navigates only through geometry it physically fits through.
- **Verification:** Define three agent configs (small: 0.3 m, medium: 0.6 m, large: 1.5 m).
  Stream in one tile region and verify all three layers load together. Pathfind a large agent
  through a narrow doorway and verify the query fails. Pathfind a small agent through the same
  doorway and verify success.

## R-7.1.13 Dynamic Area Cost Modification

The engine **SHALL** support modifying NavMesh polygon area costs at runtime without rebuilding
geometry, stored in a `NavMeshAreaCosts` ECS resource and applied during A* expansion, with
per-agent cost overrides for faction-specific routing.

- **Derived from:** [F-7.1.13](../../features/ai/navigation.md)
- **Rationale:** Runtime cost changes (flooded zones, danger areas, road buffs) must influence
  pathfinding immediately without the latency of a mesh rebuild.
- **Verification:** Set a water area cost to 10.0 at runtime. Verify agents repath to avoid
  the flooded zone on next query without any mesh rebuild occurring. Apply a faction-specific
  override reducing friendly territory cost and confirm agents of that faction prefer the
  cheaper route while other factions do not.

## R-7.1.14 Hierarchical Pathfinding (HPA*)

The engine **SHALL** build a coarse navigation graph from NavMesh tile connectivity and perform
hierarchical pathfinding that plans on the coarse graph first, then refines to full NavMesh
resolution only for the tiles the agent is currently traversing, keeping pathfinding cost bounded
regardless of world size.

- **Derived from:** [F-7.1.14](../../features/ai/navigation.md)
- **Rationale:** Per-polygon A* across an entire open world is prohibitively expensive for
  thousands of NPCs with long-distance goals; hierarchical planning bounds the cost.
- **Verification:** Issue a path query spanning 50 tiles. Measure query time and verify it is
  within 2x of a 5-tile query (demonstrating bounded cost). Confirm the coarse path visits
  the correct tile sequence and the refined path within the current tile matches full-
  resolution A*.

## R-7.1.15 NavMesh Runtime Visualization

The engine **SHALL** render NavMesh polygons, tile boundaries, area types, obstacle carve regions,
off-mesh links, and pending rebuild zones as debug overlays controlled by `NavMeshDebug` ECS
components, with color-coded area types and agent-path trails.

- **Derived from:** [F-7.1.15](../../features/ai/navigation.md)
- **Rationale:** Visual debugging is essential for designers to diagnose navigation failures in
  complex open-world geometry.
- **Verification:** Enable NavMesh debug visualization and verify polygons render with correct
  area-type colors. Place an obstacle carve and confirm the carve region is visually distinct.
  Add an off-mesh link and verify it renders as a visible connection. Trigger a tile rebuild and
  confirm the pending zone is highlighted until rebuild completes.
