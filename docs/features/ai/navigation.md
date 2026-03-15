# 7.1 — Navigation

## NavMesh Generation

### F-7.1.1 Recast-Style NavMesh Generation

Voxelizes static world geometry into a heightfield, then builds a compact polygonal navigation mesh
via watershed partitioning and contour tracing. Supports configurable agent radius, height, step
climb, and slope limits to produce meshes tailored to different entity archetypes (humanoid, mount,
large creature).

- **Requirements:** R-7.1.1
- **Dependencies:** None
- **Platform notes:** Generation runs on server; offline bake available as a content pipeline step

### F-7.1.2 NavMesh Tiling & Streaming

Divides the world NavMesh into fixed-size tiles that align with the world streaming grid. Tiles load
and unload asynchronously as the server simulation window moves, enabling navigation across a
seamless open world without holding the full mesh in memory.

- **Requirements:** R-7.1.2
- **Dependencies:** F-7.1.1
- **Platform notes:** Tile size must match or subdivide the world chunk size used by the streaming
  system

## Pathfinding

### F-7.1.3 A* Pathfinding on NavMesh

Hierarchical A* search over NavMesh polygons with configurable heuristics and cost modifiers.
Supports area-type weights (road, swamp, lava) so AI agents prefer contextually appropriate routes.
Queries are batched and spread across frames to stay within a per-tick CPU budget.

- **Requirements:** R-7.1.3
- **Dependencies:** F-7.1.1, F-7.1.2
- **Platform notes:** Per-tick pathfinding budget scales by platform: mobile 0.5 ms, Switch 1 ms,
  desktop 2 ms. Mobile limits concurrent path queries to 16; desktop allows 128+.

### F-7.1.4 String Pulling (Funnel Algorithm)

Converts a corridor of NavMesh polygons into a minimal sequence of waypoints using the funnel
algorithm. Produces tight, corner-hugging paths that avoid unnecessary detours and serve as input to
steering and path smoothing.

- **Requirements:** R-7.1.4
- **Dependencies:** F-7.1.3
- **Platform notes:** Lightweight CPU cost; runs identically on all platforms.

### F-7.1.5 Path Smoothing

Post-processes waypoint paths with short raycasts against the NavMesh to remove redundant turns and
produce natural-looking movement. Applies Catmull-Rom or cubic Bezier interpolation for curved
trajectories where aesthetics matter (e.g., town patrol routes).

- **Requirements:** R-7.1.5
- **Dependencies:** F-7.1.4
- **Platform notes:** Mobile skips Bezier interpolation and uses linear waypoint paths to reduce
  per-path CPU cost.

## Dynamic Obstacles & Links

### F-7.1.6 Dynamic Obstacle Carving

Marks NavMesh regions as temporarily blocked when dynamic obstacles (barricades, vehicles, fallen
trees) appear or move. Uses tile-local re-carving to invalidate only affected polygons, triggering
localized repath requests for agents whose corridors intersect the modified area.

- **Requirements:** R-7.1.6
- **Dependencies:** F-7.1.2
- **Platform notes:** Mobile limits simultaneous carve operations to 4 per tick; desktop allows 16+.
  Repath storms are throttled more aggressively on mobile.

### F-7.1.7 NavMesh Links (Off-Mesh Connections)

Defines traversal connections between disjoint NavMesh regions for actions like jumping across gaps,
climbing ladders, opening doors, or swimming transitions. Each link carries a cost, animation tag,
and optional precondition (key required, ability unlocked) so the planner can evaluate feasibility.

- **Requirements:** R-7.1.7
- **Dependencies:** F-7.1.1
- **Platform notes:** Link data is lightweight; runs identically on all platforms.

## Runtime NavMesh Rebuilding

### F-7.1.8 Incremental Tile Rebuild

Rebuild individual NavMesh tiles at runtime when world geometry changes — terrain deformation,
building destruction, player-placed structures, or seasonal world events. Only the affected tile and
its immediate neighbors are revoxelized and recontoured; unaffected tiles remain untouched. A
`NavMeshDirtyRegion` ECS component marks spatial extents that require rebuild, and the
`NavMeshRebuildSystem` processes them in priority order.

- **Requirements:** R-7.1.8
- **Dependencies:** F-7.1.2, F-1.1.7 (Change Detection)
- **Platform notes:** Mobile caps concurrent tile rebuilds to 1; desktop allows 4+. Mobile uses
  coarser voxel resolution (cell size 2x desktop) to reduce rebuild cost.

### F-7.1.9 Background NavMesh Generation

NavMesh tile generation and rebuild run on background worker threads via the job system (F-14.3.3),
never blocking the main simulation tick. Tiles under construction are marked as `Pending` in the
`NavMeshTileMap` ECS resource; agents with corridors through pending tiles receive a temporary
fallback (last known good mesh or straight-line movement with collision avoidance). Completed tiles
are swapped in atomically at the next sync point.

- **Requirements:** R-7.1.9
- **Dependencies:** F-7.1.8, F-14.3.3 (Job System)
- **Platform notes:** Mobile devices have fewer worker threads (2-4 vs. 8+ on desktop); generation
  queue depth is capped lower to avoid starving gameplay threads.

### F-7.1.10 Destruction-Triggered NavMesh Updates

When a `Destructible` entity fractures (F-4.6.3), the destruction system emits a
`NavMeshInvalidation` ECS event containing the affected bounding region. The `NavMeshRebuildSystem`
observes these events and enqueues incremental tile rebuilds for the affected area. Collapsed
buildings open new pathways; rubble piles create new obstacles. Rebuild priority scales with the
number of active agents whose paths intersect the region.

- **Requirements:** R-7.1.10
- **Dependencies:** F-7.1.8, F-4.6.3 (Runtime Fracture), F-1.5.1 (Events)
- **Platform notes:** Mobile may defer or skip rebuilds for distant destruction events; agents fall
  back to obstacle avoidance until the tile is rebuilt.

### F-7.1.11 Player-Built Structure Integration

Structures placed by players (housing, fortifications, barricades) register as NavMesh obstacles via
a `NavMeshObstacle` ECS component with shape data and area type override. When a structure is placed
or removed, the affected tiles are queued for incremental rebuild (F-7.1.8). Area types allow
designating player structures as impassable walls, walkable ramps, or elevated platforms with
NavMesh links auto-generated for stairs and ladders.

- **Requirements:** R-7.1.11
- **Dependencies:** F-7.1.8, F-7.1.7
- **Platform notes:** Mobile uses coarser NavMesh tile resolution (see F-7.1.8); auto-link
  generation for stairs/ladders may be deferred to reduce rebuild cost.

## Multi-Agent Navigation

### F-7.1.12 Multi-Size Agent NavMeshes

Maintain multiple NavMesh layers for different agent size classes — each defined by a
`NavMeshAgentConfig` (radius, height, step climb, max slope). A humanoid, a mounted rider, and a
siege golem each navigate on their own mesh layer. Tiles for all layers share the same spatial grid
so streaming loads all layers for a region together. Agent entities reference their layer via a
`NavMeshAgent` ECS component.

- **Requirements:** R-7.1.12
- **Dependencies:** F-7.1.1, F-7.1.2
- **Platform notes:** Mobile limits to 2 agent size layers (humanoid + large); desktop supports 4+.
  Fewer layers reduce NavMesh memory and streaming bandwidth.

### F-7.1.13 Dynamic Area Cost Modification

Modify NavMesh polygon area costs at runtime without rebuilding geometry. A flooded zone raises
water area cost; a battlefield applies danger-area weighting; a road buff reduces traversal cost.
Cost modifications are stored in a `NavMeshAreaCosts` ECS resource and applied during A* expansion.
Per-agent cost overrides allow faction-specific routing (friendly territory is cheaper).

- **Requirements:** R-7.1.13
- **Dependencies:** F-7.1.3
- **Platform notes:** Lightweight data-only operation; runs identically on all platforms.

## Long-Distance Pathfinding

### F-7.1.14 Hierarchical Pathfinding (HPA*)

A coarse navigation graph built from NavMesh tile connectivity enables O(1)-per-tile path queries
across the entire open world. Long-distance paths first plan on the hierarchical graph, then refine
to full NavMesh resolution only for the tiles the agent is currently traversing. This keeps
pathfinding cost bounded regardless of world size — critical for thousands of server- side NPCs with
cross-continent travel goals.

- **Requirements:** R-7.1.14
- **Dependencies:** F-7.1.2, F-7.1.3
- **Platform notes:** Primarily server-side. Mobile clients with local AI use HPA* exclusively for
  paths longer than 3 tiles to keep query cost minimal.

## Debugging and Visualization

### F-7.1.15 NavMesh Runtime Visualization

Render NavMesh polygons, tile boundaries, area types, obstacle carve regions, off-mesh links, and
pending rebuild zones as debug overlays. Visualization is controlled by `NavMeshDebug` ECS
components and integrates with the editor gizmo system (F-15.1.4). Color-coded area types and
agent-path trails help designers diagnose navigation failures in complex open-world geometry.

- **Requirements:** R-7.1.15
- **Dependencies:** F-7.1.1, F-15.1.4 (Gizmos)
- **Platform notes:** Debug-only feature; stripped from shipping builds on all platforms. Editor
  visualization available on desktop only.
