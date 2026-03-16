# AI Navigation Test Cases

Companion test cases for [navigation.md](navigation.md).

## Unit Tests

### TC-7.1.1.1 Voxelize Flat Plane

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100x100 flat plane at y=0, agent_radius=0.5, agent_height=2.0 | Single walkable polygon covering full surface | R-7.1.1 |
| 2 | 100x100 flat plane at y=0, cell_size=0.3 | Polygon count > 0; all polygons coplanar at y=0 | R-7.1.1 |

### TC-7.1.1.2 Slope Exclusion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ramp surface at 46 degrees, max_slope=45 | Zero walkable polygons on ramp | R-7.1.1 |
| 2 | Ramp surface at 44 degrees, max_slope=45 | Ramp included in walkable polygons | R-7.1.1 |

### TC-7.1.1.3 Agent Radius Erosion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.0m wide corridor, agent_radius=0.6 | Corridor excluded from NavMesh (too narrow) | R-7.1.1 |
| 2 | 1.0m wide corridor, agent_radius=0.3 | Corridor included in NavMesh | R-7.1.1 |

### TC-7.1.2.1 Tile Coordinate Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | World position (150.0, 0.0, 250.0), tile_size=64 | TileCoord { x: 2, z: 3 } | R-7.1.2 |
| 2 | World position (-10.0, 0.0, -10.0), tile_size=64 | TileCoord { x: -1, z: -1 } | R-7.1.2 |

### TC-7.1.2.2 Cross-Tile Path Continuity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start in tile (0,0), goal in tile (1,0) | Path crosses tile boundary with continuous corridor | R-7.1.2 |
| 2 | Start in tile (0,0), goal in tile (2,2) | Path traverses 3+ tiles without gaps | R-7.1.2 |

### TC-7.1.3.1 A* Optimal Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5x5 grid graph, uniform cost=1.0, start=(0,0), goal=(4,4) | Path cost = 8.0 (Manhattan) or 5.66 (diagonal) | R-7.1.3 |
| 2 | Graph with shortcut edge (cost=1) vs long path (cost=10) | Path uses shortcut; total cost = 1.0 | R-7.1.3 |

### TC-7.1.3.2 Area Cost Routing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two paths: road (cost=0.5, 10 polys) vs swamp (cost=3.0, 5 polys) | Path follows road despite more polygons | R-7.1.3 |
| 2 | Road cost changed from 0.5 to 5.0 at runtime | Path now follows swamp | R-7.1.3 |

### TC-7.1.3.3 Lava Impassable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Direct path crosses lava (cost=f32::INFINITY); alternate exists | Path avoids lava via alternate route | R-7.1.3 |
| 2 | All paths cross lava; no alternative | PathResult::NotFound | R-7.1.3 |

### TC-7.1.4.1 Funnel Minimal Waypoints

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Corridor of 10 polygons in a straight line | Funnel output = 2 waypoints (start, end) | R-7.1.4 |
| 2 | Corridor of 8 polygons with 2 turns | Funnel output < 8 waypoints | R-7.1.4 |

### TC-7.1.4.2 Funnel Waypoints on Mesh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-polygon corridor, funnel applied | All waypoints lie within NavMesh polygon boundaries | R-7.1.4 |
| 2 | L-shaped corridor, funnel applied | Corner waypoint lies on polygon edge, not outside mesh | R-7.1.4 |

### TC-7.1.5.1 Catmull-Rom Points on Mesh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-waypoint path, Catmull-Rom interpolation at 20 samples | All 20 samples lie on valid NavMesh polygons | R-7.1.5 |
| 2 | Path near NavMesh edge, Catmull-Rom interpolation | No interpolated point falls outside NavMesh | R-7.1.5 |

### TC-7.1.5.2 Linear Fallback Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config; request CatmullRom smoothing | Smoothing mode forced to Linear | R-7.1.5 |
| 2 | Desktop config; request CatmullRom smoothing | CatmullRom applied | R-7.1.5 |

### TC-7.1.6.1 Carve Blocks Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3x3m obstacle carved into NavMesh at chokepoint | Path through chokepoint returns NotFound | R-7.1.6 |
| 2 | 3x3m obstacle carved away from path | Original path remains valid | R-7.1.6 |

### TC-7.1.6.2 Uncarve Restores

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Carve obstacle; verify blocked; remove obstacle | Path through chokepoint restored | R-7.1.6 |
| 2 | Carve 3 overlapping obstacles; remove middle one | Remaining 2 obstacles still block | R-7.1.6 |

### TC-7.1.7.1 Link Cost Included

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Path uses off-mesh link with traversal_cost=5.0 | Path total_cost includes the 5.0 link cost | R-7.1.7 |
| 2 | Two routes: one with link (cost=5), one without (cost=8) | Path uses link (total cost lower) | R-7.1.7 |

### TC-7.1.7.2 Link Precondition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ladder link requires can_climb=true; agent can_climb=false | Path does not use ladder link | R-7.1.7 |
| 2 | Ladder link requires can_climb=true; agent can_climb=true | Path may use ladder link | R-7.1.7 |

### TC-7.1.7.3 Link One-Way

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | One-way jump-down link A->B; pathfind A to B | Path uses link | R-7.1.7 |
| 2 | One-way jump-down link A->B; pathfind B to A | Path avoids link; uses alternate route or NotFound | R-7.1.7 |

### TC-7.1.8.1 Incremental Rebuild Scope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify geometry in tile (2,3) | Only tiles (2,3) and direct neighbors rebuilt; tile (0,0) unchanged | R-7.1.8 |
| 2 | Modify geometry at tile corner (1,1)/(1,2)/(2,1)/(2,2) | All 4 tiles rebuilt; distant tiles unchanged | R-7.1.8 |

### TC-7.1.8.2 Incremental Equals Full

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify geometry; incremental rebuild | Result identical to full rebake of affected tiles | R-7.1.8 |

### TC-7.1.9.1 Pending Tile Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent navigates during tile rebuild (state=Pending) | Agent uses stale tile data as fallback | R-7.1.9 |
| 2 | Tile rebuild completes | Agent uses fresh tile data on next query | R-7.1.9 |

### TC-7.1.9.2 Atomic Tile Swap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query tile during swap operation | Returns either old or new tile; never partial data | R-7.1.9 |

### TC-7.1.10.1 Destruction Opens Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall blocks path; destroy wall | Rebuild produces passable corridor through breach | R-7.1.10 |
| 2 | Floor destroyed; creates pit | Path avoids destroyed floor area | R-7.1.10 |

### TC-7.1.10.2 Rubble Creates Obstacle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall destroyed; rubble spawned at base | Path routes around rubble pile | R-7.1.10 |

### TC-7.1.11.1 Structure Blocks Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place impassable 4x4m structure on path | Path reroutes around structure | R-7.1.11 |
| 2 | Place passable fence structure on path | Path passes through fence | R-7.1.11 |

### TC-7.1.11.2 Auto-Link Ramp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place ramp from ground (y=0) to platform (y=3) | Off-mesh link auto-generated connecting ground to platform | R-7.1.11 |

### TC-7.1.12.1 Multi-Layer Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.5m doorway; large agent (radius=1.0) queries path | Path does not traverse doorway | R-7.1.12 |
| 2 | 1.5m doorway; small agent (radius=0.3) queries path | Path traverses doorway | R-7.1.12 |

### TC-7.1.12.2 Layer Assignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent assigned to layer 0; query on layer 0 | Query succeeds; returns path | R-7.1.12 |
| 2 | Agent assigned to layer 0; query on layer 1 | Query returns NotFound (wrong layer) | R-7.1.12 |

### TC-7.1.13.1 Dynamic Cost Reroute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swamp area cost raised from 3.0 to 100.0 at runtime | Agent reroutes to avoid swamp | R-7.1.13 |
| 2 | Road area cost lowered from 1.0 to 0.1 at runtime | Agent preferentially uses road | R-7.1.13 |

### TC-7.1.13.2 Faction Cost Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Faction A: bridge cost=1.0; Faction B: bridge cost=100.0 | Faction A crosses bridge; Faction B avoids it | R-7.1.13 |

### TC-7.1.13.3 Cost Change No Rebuild

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify area cost; check tile rebuild counter | Rebuild counter unchanged (zero rebuilds) | R-7.1.13 |

### TC-7.1.14.1 HPA* Valid Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start at tile (0,0), goal at tile (49,49) | HPA* returns valid end-to-end path | R-7.1.14 |
| 2 | Start and goal in same tile | HPA* falls back to local A* | R-7.1.14 |

### TC-7.1.14.2 HPA* Bounded Cost

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50-tile HPA* query vs 5-tile A* query | 50-tile query time < 2x the 5-tile time | R-7.1.14 |

### TC-7.1.14.3 Cluster Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rebuild tile (5,5); check HPA* cluster graph | Only cluster for tile (5,5) updated; others unchanged | R-7.1.14 |

## Integration Tests

### TC-7.1.3.I1 Path Request Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add PathRequest component to entity | PathResult component produced; PathRequest auto-removed | R-7.1.3 |
| 2 | Add PathRequest for unreachable goal | PathResult with status=NotFound produced | R-7.1.3 |

### TC-7.1.6.I1 Obstacle Carve and Repath

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent following path; obstacle placed on path | Agent automatically repaths around obstacle | R-7.1.6 |
| 2 | Agent following path; obstacle placed off-path | Agent continues original path | R-7.1.6 |

### TC-7.1.10.I1 Destruction Rebuild Repath

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall destroyed; agents waiting on other side | Tile rebuilt; agents path through breach | R-7.1.10 |

### TC-7.1.2.I1 Tile Streaming Load/Unload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent moves 5 tiles forward | New tiles streamed in ahead; old tiles unloaded behind | R-7.1.2 |
| 2 | Agent teleports 20 tiles | All nearby tiles loaded; distant old tiles unloaded | R-7.1.2 |

### TC-7.1.9.I1 Background Gen No Block

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger background NavMesh generation for 10 tiles | Main-thread frame time increase < 5% | R-7.1.9 |

### TC-7.1.3.I2 Multi-Agent Parallel Queries

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 128 concurrent path queries on thread pool | All produce correct paths; ThreadSanitizer clean | R-7.1.3 |

### TC-7.1.11.I1 Structure Place/Remove Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place structure; verify blocked; remove structure | Path restored after removal | R-7.1.11 |

### TC-7.1.14.I1 HPA* Cross-Continent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 agents with cross-continent goals | All receive valid paths within server budget | R-7.1.14 |

### TC-7.1.6.I2 Carve Throttle Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config; 10 carve requests in single tick | Only 4 processed; 6 deferred to next tick | R-7.1.6 |

### TC-7.1.10.I2 Rebuild Priority Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tile A has 10 affected agents; Tile B has 2 | Tile A rebuilt before Tile B | R-7.1.10 |

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
