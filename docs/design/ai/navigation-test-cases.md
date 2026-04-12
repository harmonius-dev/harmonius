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

### TC-7.1.15.1 NavMesh Debug Overlay Renders Polygons

| # | Requirement |
|---|-------------|
| 1 | R-7.1.15    |
| 2 | R-7.1.15    |

1. **#1** — Enable debug overlay, capture editor frame with 10 NavMesh tiles
   - **Expected:** Overlay draws polygon outlines, tile boundaries, area-type shading, off-mesh
     links, and pending rebuild zones
2. **#2** — Build shipping binary and inspect
   - **Expected:** Overlay code eliminated, zero overlay symbols present

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

### TC-7.1.1.I1 Level Designer Generates NavMesh From Static Geometry

| # | Requirement |
|---|-------------|
| 1 | US-7.1.1    |
| 2 | US-7.1.1    |

1. **#1** — Level designer loads a scene with 10K static meshes and invokes NavMesh generation with
   default agent config (radius 0.4, height 1.8)
   - **Expected:** Generation completes, NavMesh covers all walkable surfaces, excludes slopes > 45
     deg
2. **#2** — Designer changes agent radius to 1.0 and regenerates
   - **Expected:** Corridors narrower than 1.0 are eroded away, confirming agent radius applied

### TC-7.1.2.I2 Level Designer Streams NavMesh Tiles With World

| # | Requirement |
|---|-------------|
| 1 | US-7.1.2    |
| 2 | US-7.1.2    |

1. **#1** — Level designer flies camera across 20 world streaming cells
   - **Expected:** NavMesh tiles align with streaming cells; new tiles load ahead, old tiles unload
     behind
2. **#2** — Inspect loaded tile count
   - **Expected:** Matches active streaming cell set, no orphan NavMesh tiles remain

### TC-7.1.3.I3 Game Designer Assigns Area Costs For Pathfinding

| # | Requirement |
|---|-------------|
| 1 | US-7.1.3    |
| 2 | US-7.1.3    |

1. **#1** — Game designer sets cost multiplier 3.0 for "swamp" area type
   - **Expected:** Subsequent path queries route through grass before swamp when both available
2. **#2** — Designer sets "lava" area to impassable
   - **Expected:** Paths never include lava polygons; alternate route returned or NotFound

### TC-7.1.4.I1 Engine Developer Converts Polygon Corridor To Waypoints

| # | Requirement |
|---|-------------|
| 1 | US-7.1.4    |
| 2 | US-7.1.4    |

1. **#1** — Engine developer runs funnel algorithm on 10-polygon corridor
   - **Expected:** Returns minimal waypoint list, each waypoint on NavMesh, path length within 5 %
     of optimal
2. **#2** — Run funnel on straight corridor
   - **Expected:** Returns only start and end waypoints

### TC-7.1.5.I1 Game Designer Smooths Patrol Paths With Catmull-Rom

| # | Requirement |
|---|-------------|
| 1 | US-7.1.5    |
| 2 | US-7.1.5    |

1. **#1** — Game designer enables Catmull-Rom smoothing on 8-waypoint patrol path
   - **Expected:** Smoothed points projected onto NavMesh surface, no point off-mesh
2. **#2** — Enable mobile profile
   - **Expected:** Engine falls back to linear interpolation, smoothing cost under budget

### TC-7.1.6.I3 Game Designer Places Dynamic Barricade Obstacle

| # | Requirement |
|---|-------------|
| 1 | US-7.1.6    |
| 2 | US-7.1.6    |

1. **#1** — Game designer places barricade prefab at runtime on open NavMesh
   - **Expected:** Obstacle carve makes barricade AABB impassable within 1 frame
2. **#2** — Remove barricade
   - **Expected:** Uncarve restores original NavMesh polygons; existing paths through area re-valid

### TC-7.1.7.I1 Level Designer Places Off-Mesh Jump Link

| # | Requirement |
|---|-------------|
| 1 | US-7.1.7    |
| 2 | US-7.1.7    |

1. **#1** — Level designer places an off-mesh jump link between two ledges in editor
   - **Expected:** Path queries traverse the link when it offers lower cost
2. **#2** — Designer sets link to one-way
   - **Expected:** Query only traverses from start to end, reverse path avoids link

### TC-7.1.8.I1 Game Designer Triggers Incremental NavMesh Rebuild

| # | Requirement |
|---|-------------|
| 1 | US-7.1.8    |
| 2 | US-7.1.8    |

1. **#1** — Game designer modifies 1 tile of static geometry at runtime
   - **Expected:** Only that tile and direct neighbours rebuild, other tiles untouched
2. **#2** — Compare incremental rebuild output to a full regen of the same world
   - **Expected:** NavMesh polygons equivalent within tolerance

### TC-7.1.9.I2 Engine Developer Runs NavMesh Generation In Background

| # | Requirement |
|---|-------------|
| 1 | US-7.1.9    |
| 2 | US-7.1.9    |

1. **#1** — Engine developer triggers 20-tile NavMesh generation while game is running at 60 FPS
   - **Expected:** Main thread frame time increases by less than 5 %, generation completes on worker
     threads
2. **#2** — Inspect worker thread stacks
   - **Expected:** NavMesh generation runs only on worker pool, not main thread

### TC-7.1.10.I3 Game Designer Destroys Building To Open Path

| # | Requirement |
|---|-------------|
| 1 | US-7.1.10   |
| 2 | US-7.1.10   |

1. **#1** — Game designer destroys a wall blocking agent path
   - **Expected:** Destruction emits nav rebuild event, affected tile rebuilt, agent repaths through
     breach
2. **#2** — Rubble debris remains
   - **Expected:** Debris obstacle carved out of NavMesh, agent paths around rubble

### TC-7.1.11.I2 Level Designer Places Player-Built Structure

| # | Requirement |
|---|-------------|
| 1 | US-7.1.11   |
| 2 | US-7.1.11   |

1. **#1** — Level designer places a player-built ramp structure at runtime
   - **Expected:** Structure registers with nav system, tile rebuild produces walkable ramp polygons
     and auto-link
2. **#2** — Agent pathing from below ramp to above
   - **Expected:** Path uses the new auto-generated link over the ramp

### TC-7.1.12.I1 Game Designer Separates NavMesh Layers By Agent Type

| # | Requirement |
|---|-------------|
| 1 | US-7.1.12   |
| 2 | US-7.1.12   |

1. **#1** — Game designer configures humanoid and quadruped layers with different agent radii
   - **Expected:** Each layer generated on shared source geometry, separate polygon sets
2. **#2** — Path query on humanoid layer
   - **Expected:** Uses humanoid polygons only, quadruped polygons not considered

### TC-7.1.13.I1 Game Designer Applies Dynamic Area Cost Override

| # | Requirement |
|---|-------------|
| 1 | US-7.1.13   |
| 2 | US-7.1.13   |

1. **#1** — Game designer raises cost of "flooded" area at runtime
   - **Expected:** Next path query reroutes around flooded polygons without NavMesh rebuild
2. **#2** — Apply per-faction cost override for "enemy territory"
   - **Expected:** Enemy agents route directly; friendly agents avoid the area

### TC-7.1.14.I2 Game Developer Uses HPA* For Long Paths

| # | Requirement |
|---|-------------|
| 1 | US-7.1.14   |
| 2 | US-7.1.14   |

1. **#1** — Game developer requests path from 1000 agents across continent-scale world
   - **Expected:** HPA* hierarchical planner produces path in under server per-frame budget
2. **#2** — Compare HPA* path cost to flat A* reference
   - **Expected:** HPA* path cost within bounded ratio (e.g. 1.1x) of optimum

### TC-7.1.15.I1 Technical Artist Views NavMesh Debug Overlay In Editor

| # | Requirement |
|---|-------------|
| 1 | US-7.1.15   |
| 2 | US-7.1.15   |

1. **#1** — Technical artist enables NavMesh debug overlay in editor viewport
   - **Expected:** Polygons, boundaries, area types, off-mesh links, and pending rebuild zones
     render as colored overlays
2. **#2** — Toggle each overlay category independently
   - **Expected:** Each subset shows and hides without affecting the others

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

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-7.1.1.1
- US-7.1.1.12
- US-7.1.10.1
- US-7.1.10.12
- US-7.1.11.1
- US-7.1.11.12
- US-7.1.12.1
- US-7.1.12.12
- US-7.1.13.1
- US-7.1.13.12
- US-7.1.14.1
- US-7.1.14.12
- US-7.1.14.3
- US-7.1.15.1
- US-7.1.15.12
- US-7.1.2.1
- US-7.1.2.12
- US-7.1.3.1
- US-7.1.3.12
- US-7.1.3.4
- US-7.1.4.1
- US-7.1.4.12
- US-7.1.5.1
- US-7.1.5.11
- US-7.1.5.12
- US-7.1.6.1
- US-7.1.6.12
- US-7.1.6.7
- US-7.1.7.1
- US-7.1.7.12
- US-7.1.7.4
- US-7.1.7.5
- US-7.1.8.1
- US-7.1.8.12
- US-7.1.9.1
- US-7.1.9.12
- US-7.1.9.3
