# AI Navigation Test Cases

Companion test cases for [navigation.md](navigation.md).

## Unit Tests

### TC-7.1.1.1 Voxelize Flat Plane

| # | Requirement |
|---|-------------|
| 1 | R-7.1.1     |
| 2 | R-7.1.1     |

1. **#1** — 100x100 flat plane at y=0, agent_radius=0.5, agent_height=2.0
   - **Expected:** Single walkable polygon covering full surface
2. **#2** — 100x100 flat plane at y=0, cell_size=0.3
   - **Expected:** Polygon count > 0; all polygons coplanar at y=0

### TC-7.1.1.2 Slope Exclusion

| # | Requirement |
|---|-------------|
| 1 | R-7.1.1     |
| 2 | R-7.1.1     |

1. **#1** — Ramp surface at 46 degrees, max_slope=45
   - **Expected:** Zero walkable polygons on ramp
2. **#2** — Ramp surface at 44 degrees, max_slope=45
   - **Expected:** Ramp included in walkable polygons

### TC-7.1.1.3 Agent Radius Erosion

| # | Requirement |
|---|-------------|
| 1 | R-7.1.1     |
| 2 | R-7.1.1     |

1. **#1** — 1.0m wide corridor, agent_radius=0.6
   - **Expected:** Corridor excluded from NavMesh (too narrow)
2. **#2** — 1.0m wide corridor, agent_radius=0.3
   - **Expected:** Corridor included in NavMesh

### TC-7.1.2.1 Tile Coordinate Mapping

| # | Requirement |
|---|-------------|
| 1 | R-7.1.2     |
| 2 | R-7.1.2     |

1. **#1** — World position (150.0, 0.0, 250.0), tile_size=64
   - **Expected:** TileCoord { x: 2, z: 3 }
2. **#2** — World position (-10.0, 0.0, -10.0), tile_size=64
   - **Expected:** TileCoord { x: -1, z: -1 }

### TC-7.1.2.2 Cross-Tile Path Continuity

| # | Requirement |
|---|-------------|
| 1 | R-7.1.2     |
| 2 | R-7.1.2     |

1. **#1** — Start in tile (0,0), goal in tile (1,0)
   - **Expected:** Path crosses tile boundary with continuous corridor
2. **#2** — Start in tile (0,0), goal in tile (2,2)
   - **Expected:** Path traverses 3+ tiles without gaps

### TC-7.1.3.1 A* Optimal Path

| # | Requirement |
|---|-------------|
| 1 | R-7.1.3     |
| 2 | R-7.1.3     |

1. **#1** — 5x5 grid graph, uniform cost=1.0, start=(0,0), goal=(4,4)
   - **Expected:** Path cost = 8.0 (Manhattan) or 5.66 (diagonal)
2. **#2** — Graph with shortcut edge (cost=1) vs long path (cost=10)
   - **Expected:** Path uses shortcut; total cost = 1.0

### TC-7.1.3.2 Area Cost Routing

| # | Requirement |
|---|-------------|
| 1 | R-7.1.3     |
| 2 | R-7.1.3     |

1. **#1** — Two paths: road (cost=0.5, 10 polys) vs swamp (cost=3.0, 5 polys)
   - **Expected:** Path follows road despite more polygons
2. **#2** — Road cost changed from 0.5 to 5.0 at runtime
   - **Expected:** Path now follows swamp

### TC-7.1.3.3 Lava Impassable

| # | Requirement |
|---|-------------|
| 1 | R-7.1.3     |
| 2 | R-7.1.3     |

1. **#1** — Direct path crosses lava (cost=f32::INFINITY); alternate exists
   - **Expected:** Path avoids lava via alternate route
2. **#2** — All paths cross lava; no alternative
   - **Expected:** PathResult::NotFound

### TC-7.1.4.1 Funnel Minimal Waypoints

| # | Requirement |
|---|-------------|
| 1 | R-7.1.4     |
| 2 | R-7.1.4     |

1. **#1** — Corridor of 10 polygons in a straight line
   - **Expected:** Funnel output = 2 waypoints (start, end)
2. **#2** — Corridor of 8 polygons with 2 turns
   - **Expected:** Funnel output < 8 waypoints

### TC-7.1.4.2 Funnel Waypoints on Mesh

| # | Requirement |
|---|-------------|
| 1 | R-7.1.4     |
| 2 | R-7.1.4     |

1. **#1** — 20-polygon corridor, funnel applied
   - **Expected:** All waypoints lie within NavMesh polygon boundaries
2. **#2** — L-shaped corridor, funnel applied
   - **Expected:** Corner waypoint lies on polygon edge, not outside mesh

### TC-7.1.5.1 Catmull-Rom Points on Mesh

| # | Requirement |
|---|-------------|
| 1 | R-7.1.5     |
| 2 | R-7.1.5     |

1. **#1** — 5-waypoint path, Catmull-Rom interpolation at 20 samples
   - **Expected:** All 20 samples lie on valid NavMesh polygons
2. **#2** — Path near NavMesh edge, Catmull-Rom interpolation
   - **Expected:** No interpolated point falls outside NavMesh

### TC-7.1.5.2 Linear Fallback Mobile

| # | Requirement |
|---|-------------|
| 1 | R-7.1.5     |
| 2 | R-7.1.5     |

1. **#1** — Mobile config; request CatmullRom smoothing
   - **Expected:** Smoothing mode forced to Linear
2. **#2** — Desktop config; request CatmullRom smoothing
   - **Expected:** CatmullRom applied

### TC-7.1.6.1 Carve Blocks Path

| # | Requirement |
|---|-------------|
| 1 | R-7.1.6     |
| 2 | R-7.1.6     |

1. **#1** — 3x3m obstacle carved into NavMesh at chokepoint
   - **Expected:** Path through chokepoint returns NotFound
2. **#2** — 3x3m obstacle carved away from path
   - **Expected:** Original path remains valid

### TC-7.1.6.2 Uncarve Restores

| # | Requirement |
|---|-------------|
| 1 | R-7.1.6     |
| 2 | R-7.1.6     |

1. **#1** — Carve obstacle; verify blocked; remove obstacle
   - **Expected:** Path through chokepoint restored
2. **#2** — Carve 3 overlapping obstacles; remove middle one
   - **Expected:** Remaining 2 obstacles still block

### TC-7.1.7.1 Link Cost Included

| # | Requirement |
|---|-------------|
| 1 | R-7.1.7     |
| 2 | R-7.1.7     |

1. **#1** — Path uses off-mesh link with traversal_cost=5.0
   - **Expected:** Path total_cost includes the 5.0 link cost
2. **#2** — Two routes: one with link (cost=5), one without (cost=8)
   - **Expected:** Path uses link (total cost lower)

### TC-7.1.7.2 Link Precondition

| # | Requirement |
|---|-------------|
| 1 | R-7.1.7     |
| 2 | R-7.1.7     |

1. **#1** — Ladder link requires can_climb=true; agent can_climb=false
   - **Expected:** Path does not use ladder link
2. **#2** — Ladder link requires can_climb=true; agent can_climb=true
   - **Expected:** Path may use ladder link

### TC-7.1.7.3 Link One-Way

| # | Requirement |
|---|-------------|
| 1 | R-7.1.7     |
| 2 | R-7.1.7     |

1. **#1** — One-way jump-down link A->B; pathfind A to B
   - **Expected:** Path uses link
2. **#2** — One-way jump-down link A->B; pathfind B to A
   - **Expected:** Path avoids link; uses alternate route or NotFound

### TC-7.1.8.1 Incremental Rebuild Scope

| # | Requirement |
|---|-------------|
| 1 | R-7.1.8     |
| 2 | R-7.1.8     |

1. **#1** — Modify geometry in tile (2,3)
   - **Expected:** Only tiles (2,3) and direct neighbors rebuilt; tile (0,0) unchanged
2. **#2** — Modify geometry at tile corner (1,1)/(1,2)/(2,1)/(2,2)
   - **Expected:** All 4 tiles rebuilt; distant tiles unchanged

### TC-7.1.8.2 Incremental Equals Full

| # | Requirement |
|---|-------------|
| 1 | R-7.1.8     |

1. **#1** — Modify geometry; incremental rebuild
   - **Expected:** Result identical to full rebake of affected tiles

### TC-7.1.9.1 Pending Tile Fallback

| # | Requirement |
|---|-------------|
| 1 | R-7.1.9     |
| 2 | R-7.1.9     |

1. **#1** — Agent navigates during tile rebuild (state=Pending)
   - **Expected:** Agent uses stale tile data as fallback
2. **#2** — Tile rebuild completes
   - **Expected:** Agent uses fresh tile data on next query

### TC-7.1.9.2 Atomic Tile Swap

| # | Requirement |
|---|-------------|
| 1 | R-7.1.9     |

1. **#1** — Query tile during swap operation
   - **Expected:** Returns either old or new tile; never partial data

### TC-7.1.10.1 Destruction Opens Path

| # | Requirement |
|---|-------------|
| 1 | R-7.1.10    |
| 2 | R-7.1.10    |

1. **#1** — Wall blocks path; destroy wall
   - **Expected:** Rebuild produces passable corridor through breach
2. **#2** — Floor destroyed; creates pit
   - **Expected:** Path avoids destroyed floor area

### TC-7.1.10.2 Rubble Creates Obstacle

| # | Requirement |
|---|-------------|
| 1 | R-7.1.10    |

1. **#1** — Wall destroyed; rubble spawned at base
   - **Expected:** Path routes around rubble pile

### TC-7.1.11.1 Structure Blocks Path

| # | Requirement |
|---|-------------|
| 1 | R-7.1.11    |
| 2 | R-7.1.11    |

1. **#1** — Place impassable 4x4m structure on path
   - **Expected:** Path reroutes around structure
2. **#2** — Place passable fence structure on path
   - **Expected:** Path passes through fence

### TC-7.1.11.2 Auto-Link Ramp

| # | Requirement |
|---|-------------|
| 1 | R-7.1.11    |

1. **#1** — Place ramp from ground (y=0) to platform (y=3)
   - **Expected:** Off-mesh link auto-generated connecting ground to platform

### TC-7.1.12.1 Multi-Layer Isolation

| # | Requirement |
|---|-------------|
| 1 | R-7.1.12    |
| 2 | R-7.1.12    |

1. **#1** — 1.5m doorway; large agent (radius=1.0) queries path
   - **Expected:** Path does not traverse doorway
2. **#2** — 1.5m doorway; small agent (radius=0.3) queries path
   - **Expected:** Path traverses doorway

### TC-7.1.12.2 Layer Assignment

| # | Requirement |
|---|-------------|
| 1 | R-7.1.12    |
| 2 | R-7.1.12    |

1. **#1** — Agent assigned to layer 0; query on layer 0
   - **Expected:** Query succeeds; returns path
2. **#2** — Agent assigned to layer 0; query on layer 1
   - **Expected:** Query returns NotFound (wrong layer)

### TC-7.1.13.1 Dynamic Cost Reroute

| # | Requirement |
|---|-------------|
| 1 | R-7.1.13    |
| 2 | R-7.1.13    |

1. **#1** — Swamp area cost raised from 3.0 to 100.0 at runtime
   - **Expected:** Agent reroutes to avoid swamp
2. **#2** — Road area cost lowered from 1.0 to 0.1 at runtime
   - **Expected:** Agent preferentially uses road

### TC-7.1.13.2 Faction Cost Override

| # | Requirement |
|---|-------------|
| 1 | R-7.1.13    |

1. **#1** — Faction A: bridge cost=1.0; Faction B: bridge cost=100.0
   - **Expected:** Faction A crosses bridge; Faction B avoids it

### TC-7.1.13.3 Cost Change No Rebuild

| # | Requirement |
|---|-------------|
| 1 | R-7.1.13    |

1. **#1** — Modify area cost; check tile rebuild counter
   - **Expected:** Rebuild counter unchanged (zero rebuilds)

### TC-7.1.14.1 HPA* Valid Path

| # | Requirement |
|---|-------------|
| 1 | R-7.1.14    |
| 2 | R-7.1.14    |

1. **#1** — Start at tile (0,0), goal at tile (49,49)
   - **Expected:** HPA* returns valid end-to-end path
2. **#2** — Start and goal in same tile
   - **Expected:** HPA* falls back to local A*

### TC-7.1.14.2 HPA* Bounded Cost

| # | Requirement |
|---|-------------|
| 1 | R-7.1.14    |

1. **#1** — 50-tile HPA* query vs 5-tile A* query
   - **Expected:** 50-tile query time < 2x the 5-tile time

### TC-7.1.14.3 Cluster Update

| # | Requirement |
|---|-------------|
| 1 | R-7.1.14    |

1. **#1** — Rebuild tile (5,5); check HPA* cluster graph
   - **Expected:** Only cluster for tile (5,5) updated; others unchanged

## Integration Tests

### TC-7.1.3.I1 Path Request Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-7.1.3     |
| 2 | R-7.1.3     |

1. **#1** — Add PathRequest component to entity
   - **Expected:** PathResult component produced; PathRequest auto-removed
2. **#2** — Add PathRequest for unreachable goal
   - **Expected:** PathResult with status=NotFound produced

### TC-7.1.6.I1 Obstacle Carve and Repath

| # | Requirement |
|---|-------------|
| 1 | R-7.1.6     |
| 2 | R-7.1.6     |

1. **#1** — Agent following path; obstacle placed on path
   - **Expected:** Agent automatically repaths around obstacle
2. **#2** — Agent following path; obstacle placed off-path
   - **Expected:** Agent continues original path

### TC-7.1.10.I1 Destruction Rebuild Repath

| # | Requirement |
|---|-------------|
| 1 | R-7.1.10    |

1. **#1** — Wall destroyed; agents waiting on other side
   - **Expected:** Tile rebuilt; agents path through breach

### TC-7.1.2.I1 Tile Streaming Load/Unload

| # | Requirement |
|---|-------------|
| 1 | R-7.1.2     |
| 2 | R-7.1.2     |

1. **#1** — Agent moves 5 tiles forward
   - **Expected:** New tiles streamed in ahead; old tiles unloaded behind
2. **#2** — Agent teleports 20 tiles
   - **Expected:** All nearby tiles loaded; distant old tiles unloaded

### TC-7.1.9.I1 Background Gen No Block

| # | Requirement |
|---|-------------|
| 1 | R-7.1.9     |

1. **#1** — Trigger background NavMesh generation for 10 tiles
   - **Expected:** Main-thread frame time increase < 5%

### TC-7.1.3.I2 Multi-Agent Parallel Queries

| # | Requirement |
|---|-------------|
| 1 | R-7.1.3     |

1. **#1** — 128 concurrent path queries on thread pool
   - **Expected:** All produce correct paths; ThreadSanitizer clean

### TC-7.1.11.I1 Structure Place/Remove Cycle

| # | Requirement |
|---|-------------|
| 1 | R-7.1.11    |

1. **#1** — Place structure; verify blocked; remove structure
   - **Expected:** Path restored after removal

### TC-7.1.14.I1 HPA* Cross-Continent

| # | Requirement |
|---|-------------|
| 1 | R-7.1.14    |

1. **#1** — 1000 agents with cross-continent goals
   - **Expected:** All receive valid paths within server budget

### TC-7.1.6.I2 Carve Throttle Mobile

| # | Requirement |
|---|-------------|
| 1 | R-7.1.6     |

1. **#1** — Mobile config; 10 carve requests in single tick
   - **Expected:** Only 4 processed; 6 deferred to next tick

### TC-7.1.10.I2 Rebuild Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-7.1.10    |

1. **#1** — Tile A has 10 affected agents; Tile B has 2
   - **Expected:** Tile A rebuilt before Tile B

## Benchmarks

### TC-7.1.1.B1 NavMesh Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single tile generation (64x64m, mixed geometry) | Wall time | < 50 ms | R-7.1.1 |

### TC-7.1.3.B1 A* Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10-tile path, uniform cost | Wall time | < 0.1 ms | R-7.1.3 |
| 2 | 10-tile path, mixed area costs | Wall time | < 0.1 ms | R-7.1.3 |

### TC-7.1.4.B1 Funnel Algorithm

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 20-polygon corridor | Wall time | < 0.01 ms | R-7.1.4 |

### TC-7.1.5.B1 Path Smoothing

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 agents, Catmull-Rom smoothing | Total wall time | < 0.5 ms | R-7.1.5 |

### TC-7.1.6.B1 Obstacle Carve

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single obstacle carve/uncarve cycle | Wall time | < 0.05 ms | R-7.1.6 |

### TC-7.1.8.B1 Incremental Tile Rebuild

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single tile incremental rebuild | Wall time | < 5 ms | R-7.1.8 |

### TC-7.1.14.B1 HPA* Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50-tile HPA* query | Wall time | < 0.2 ms | R-7.1.14 |

### TC-7.1.2.B1 Tile Streaming I/O

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single tile async load from disk | Wall time | < 2 ms | R-7.1.2 |

### TC-7.1.3.B2 Concurrent Queries

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 128 concurrent path queries, desktop | Total wall time | < 2 ms | R-7.1.3 |

### TC-7.1.14.B2 HPA* Server Scale

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 HPA* queries, server | Total wall time | < 4 ms | R-7.1.14 |
