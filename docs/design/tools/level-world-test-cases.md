# Level Editor and World Building Test Cases

Companion test cases for [level-world.md](level-world.md).

## Unit Tests

### TC-15.2.1.1 Grid Snap Alignment

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |
| 2 | R-15.2.1    |
| 3 | R-15.2.1    |

1. **#1** — Position `(1.3, 0, 2.7)`, `SnapMode::Grid { cell_size: 1.0 }`
   - **Expected:** Snapped position `(1.0, 0.0, 3.0)`
2. **#2** — Position `(2.5, 0, 4.5)`, `SnapMode::Grid { cell_size: 0.5 }`
   - **Expected:** Snapped position `(2.5, 0.0, 4.5)`
3. **#3** — Position `(0.1, 0, 0.1)`, `SnapMode::Grid { cell_size: 1.0 }`
   - **Expected:** Snapped position `(0.0, 0.0, 0.0)`

### TC-15.2.1.2 Surface Snap Terrain

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |

1. **#1** — Raycast onto terrain slope at 30 degrees
   - **Expected:** Entity aligns to surface normal (rotation ~30 degrees from vertical)

### TC-15.2.1.3 Vertex Snap Precision

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |

1. **#1** — Snap to vertex at `[5.0, 3.0, 7.0]`
   - **Expected:** Snapped position within epsilon `1e-5` of `[5.0, 3.0, 7.0]`

### TC-15.2.2.1 Entity Template Instantiate

| # | Requirement |
|---|-------------|
| 1 | R-15.2.2    |

1. **#1** — Instantiate 3-level nested entity template (root -> child -> grandchild)
   - **Expected:** All 3 entities spawned with correct parent-child hierarchy

### TC-15.2.2.2 Entity Template Propagation

| # | Requirement |
|---|-------------|
| 1 | R-15.2.2    |
| 2 | R-15.2.2    |

1. **#1** — Modify source entity template `hp: 100 -> 200`, instance without override
   - **Expected:** Instance `hp == 200`
2. **#2** — Modify source entity template `hp: 100 -> 200`, instance with `hp` overridden to 50
   - **Expected:** Instance `hp == 50` (override preserved)

### TC-15.2.3.1 Override Set Revert

| # | Requirement |
|---|-------------|
| 1 | R-15.2.3    |
| 2 | R-15.2.3    |

1. **#1** — `override.set(path, value_B)` where source is value_A
   - **Expected:** `has_override(path) == true`, entity has value_B
2. **#2** — `revert_property(path)`
   - **Expected:** `has_override(path) == false`, entity has value_A

### TC-15.2.3.2 Override Apply to Source

| # | Requirement |
|---|-------------|
| 1 | R-15.2.3    |

1. **#1** — Override `hp = 150`, `apply_to_source(path)`
   - **Expected:** Source entity template `hp == 150`, all non-overridden instances update to 150

### TC-15.2.4.1 CSG Boolean Additive

| # | Requirement |
|---|-------------|
| 1 | R-15.2.4    |

1. **#1** — Two `Box { extents: [1,1,1] }` brushes overlapping, `boolean_op(a, b)`
   - **Expected:** Result is watertight mesh, no degenerate triangles

### TC-15.2.4.2 CSG Boolean Subtractive

| # | Requirement |
|---|-------------|
| 1 | R-15.2.4    |

1. **#1** — `Box { extents: [2,2,2] }` minus `Cylinder { radius: 0.5, height: 3.0 }`
   - **Expected:** Result mesh has cylindrical hole, watertight

### TC-15.2.4.3 CSG to Static Mesh

| # | Requirement |
|---|-------------|
| 1 | R-15.2.4    |

1. **#1** — `convert_to_static_mesh(brush)`
   - **Expected:** Returns valid `AssetId`, mesh asset registered

### TC-15.2.5.1 Spline Bezier C1

| # | Requirement |
|---|-------------|
| 1 | R-15.2.5    |

1. **#1** — Create Bezier spline with 4 control points
   - **Expected:** First derivative continuous at all control points (C1 continuity)

### TC-15.2.5.2 Spline Distribute

| # | Requirement |
|---|-------------|
| 1 | R-15.2.5    |

1. **#1** — `distribute_along(spline, DistributeConfig { spacing: 1.0, .. })`, spline length 10
   - **Expected:** 10 entities placed at ~1.0 unit intervals

### TC-15.2.6.1 Landscape Auto Paint

| # | Requirement |
|---|-------------|
| 1 | R-15.2.6    |

1. **#1** — Rule: layer 0 for slope 0-30, layer 1 for slope 30-60
   - **Expected:** Terrain at 20 degrees has layer 0, terrain at 45 degrees has layer 1

### TC-15.2.6.2 Landscape Weight Sum

| # | Requirement |
|---|-------------|
| 1 | R-15.2.6    |

1. **#1** — Paint layer 0 at 0.6, paint layer 1 at 0.4, check weights
   - **Expected:** Weights sum to 1.0 per texel (within epsilon)

### TC-15.2.7.1 Foliage Slope Limit

| # | Requirement |
|---|-------------|
| 1 | R-15.2.7    |

1. **#1** — `FoliagePlacementRule { max_slope: 30.0, .. }`, terrain has 45-degree region
   - **Expected:** No foliage instances placed on 45-degree slopes

### TC-15.2.7.2 Foliage Exclusion Zone

| # | Requirement |
|---|-------------|
| 1 | R-15.2.7    |

1. **#1** — `add_exclusion_zone(bounds)`, paint foliage over zone
   - **Expected:** No foliage instances within exclusion bounds

### TC-15.6.1.1 Sculpt Raise Lower

| # | Requirement |
|---|-------------|
| 1 | R-15.6.1    |
| 2 | R-15.6.1    |

1. **#1** — Apply `SculptBrushType::Raise` at center
   - **Expected:** Heightmap values at brush center increased
2. **#2** — Apply `SculptBrushType::Lower` at same center
   - **Expected:** Heightmap values decreased below raised level

### TC-15.6.1.2 Sculpt Streaming

| # | Requirement |
|---|-------------|
| 1 | R-15.6.1    |

1. **#1** — Sculpt across 16k x 16k heightmap
   - **Expected:** Peak memory usage < 512 MB

### TC-15.6.2.1 Erosion Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-15.6.2    |

1. **#1** — `simulate(region, Hydraulic, params)` twice with same params
   - **Expected:** Identical heightmap output both runs

### TC-15.6.3.1 Terrain material weight map paint

| # | Requirement |
|---|-------------|
| 1 | R-15.6.3    |
| 2 | R-15.6.3    |

1. **#1** — `paint_weight(region, layer_id=2, radius=5.0, strength=1.0)` at center (10, 10)
   - **Expected:** Weight-map channel 2 at (10, 10) equals 1.0; neighboring cells fall off per brush
     curve.
2. **#2** — Sum all layer weights at any cell after paint.
   - **Expected:** Sum equals 1.0 within tolerance 1e-5 (normalized across layers).

### TC-15.6.4.1 Water River Spline

| # | Requirement |
|---|-------------|
| 1 | R-15.6.4    |

1. **#1** — `create_river(spline, RiverConfig { width: 5.0, depth: 2.0, .. })`
   - **Expected:** Entity spawned with water components, following spline path

### TC-15.6.4.2 Water Lake Altitude

| # | Requirement |
|---|-------------|
| 1 | R-15.6.4    |

1. **#1** — `create_lake(boundary, LakeConfig { surface_altitude: 100.0, .. })`
   - **Expected:** Water surface entity at altitude 100.0

### TC-15.6.5.1 Biome Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-15.6.5    |

1. **#1** — `auto_populate(region, rules, seed: 42)` twice
   - **Expected:** Same entity count and positions both runs

### TC-15.6.5.2 Biome Rule Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.6.5    |

1. **#1** — `auto_populate(region, rules)`, `validate_placements(region, rules)`
   - **Expected:** Returns empty violations list

### TC-15.6.6.1 Probe Grid Placement

| # | Requirement |
|---|-------------|
| 1 | R-15.6.6    |

1. **#1** — `place_light_probe_grid(bounds, TetrahedralGrid { spacing: 5.0 })` in 20x20x20 area
   - **Expected:** Probes placed at ~5 unit spacing, count approximately 64

### TC-15.6.7.1 Navmesh Slope Limit

| # | Requirement |
|---|-------------|
| 1 | R-15.6.7    |

1. **#1** — Generate navmesh, terrain has 60-degree slope region
   - **Expected:** Steep region marked non-walkable in navmesh

### TC-15.6.8.1 Partition Budget

| # | Requirement |
|---|-------------|
| 1 | R-15.6.8    |

1. **#1** — Cell with `entity_budget: 100`, add 150 entities
   - **Expected:** `CellBudgetStatus { over_budget: true, entity_count: 150 }`

### TC-15.6.8.2 Multiuser Lock

| # | Requirement |
|---|-------------|
| 1 | R-15.6.8    |
| 2 | R-15.6.8    |

1. **#1** — User A `request_lock(cell(3,7))`
   - **Expected:** Returns `Ok(CellLock)`
2. **#2** — User B `request_lock(cell(3,7))` while A holds lock
   - **Expected:** Returns `Err(LockError::CellOwnedBy(user_A))`

## Integration Tests

### TC-15.2.1.I1 Place Undo Redo

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |
| 2 | R-15.2.1    |

1. **#1** — Place entity via EntityPlacer, undo
   - **Expected:** Entity removed from ECS world
2. **#2** — Redo after undo
   - **Expected:** Entity restored at same position

### TC-15.2.2.I1 Entity Template Save Load Round Trip

| # | Requirement |
|---|-------------|
| 1 | R-15.2.2    |

1. **#1** — Create nested entity template, save to disk, load
   - **Expected:** Loaded entity template structure identical to saved

### TC-15.2.4.I1 CSG Render Collision

| # | Requirement |
|---|-------------|
| 1 | R-15.2.4    |

1. **#1** — CSG union of two boxes, render and generate collision
   - **Expected:** Visual mesh renders correctly, collision mesh matches visual

### TC-15.6.1.I1 Terrain Sculpt Async IO

| # | Requirement |
|---|-------------|
| 1 | R-15.6.1    |

1. **#1** — Sculpt terrain with async I/O enabled
   - **Expected:** No worker threads blocked during sculpting, UI remains responsive

### TC-15.6.2.I1 Erosion GPU 15fps

| # | Requirement |
|---|-------------|
| 1 | R-15.6.2    |

1. **#1** — Run erosion preview on 2048x2048 terrain region
   - **Expected:** Preview framerate stays above 15 FPS

### TC-15.6.8.I1 Multiuser Concurrent

| # | Requirement |
|---|-------------|
| 1 | R-15.6.8    |

1. **#1** — User A edits cell (2,3), user B edits cell (5,7) concurrently
   - **Expected:** Both changes persist after sync

### TC-15.6.8.I2 Partition Streaming

| # | Requirement |
|---|-------------|
| 1 | R-15.6.8    |

1. **#1** — Load cell (1,1), unload cell (0,0)
   - **Expected:** Cell (1,1) state is `Loaded`, cell (0,0) state is `Unloaded` in overlay

### TC-15.2.1.I2 Place with grid, surface, and vertex snap

| # | Requirement |
|---|-------------|
| 1 | US-15.2.1   |

1. **#1** — Place an entity three times at the same cursor, each with a different `SnapMode`
   (`Grid { cell_size: 1.0 }`, `Surface`, `Vertex`).
   - **Expected:** Grid place snaps to `(1,0,1)`; surface place aligns to terrain surface normal;
     vertex place snaps to nearest mesh vertex within 0.01 m.

### TC-15.2.2.I1 Nested entity template instantiation

| # | Requirement |
|---|-------------|
| 1 | US-15.2.2   |

1. **#1** — Create a parent template containing a nested child template; instantiate the parent
   twice; mutate the child template.
   - **Expected:** Both parent instances receive the mutation in the child; hierarchy structure
     preserved.

### TC-15.2.4.I1 CSG brush blockout to static mesh

| # | Requirement |
|---|-------------|
| 1 | US-15.2.4   |

1. **#1** — Use additive and subtractive brushes to construct a blockout, then convert to static
   mesh.
   - **Expected:** Resulting static mesh has closed manifold surface; boolean result matches the
     visual preview.

### TC-15.2.7.I1 Foliage paint with density and rules

| # | Requirement |
|---|-------------|
| 1 | US-15.2.7   |

1. **#1** — Paint foliage over a 50x50 region with density 10/m^2 and exclusion rule for slopes > 45
   degrees.
   - **Expected:** Instances placed only on surfaces with slope <= 45 degrees; average density on
     valid regions equals 10/m^2 within 10% tolerance.

### TC-15.6.1.I1 Terrain sculpt brush with streaming

| # | Requirement |
|---|-------------|
| 1 | US-15.6.1   |

1. **#1** — Sculpt a raise brush over a region spanning two streaming chunks.
   - **Expected:** Both chunks updated and written to disk via platform-native I/O; brush stroke
     visible seamlessly across chunk boundary.

### TC-15.6.2.I1 Hydraulic and thermal GPU erosion

| # | Requirement |
|---|-------------|
| 1 | US-15.6.2   |

1. **#1** — Simulate 100 iterations of hydraulic erosion followed by 50 of thermal on a 1024x1024
   heightmap via GPU.
   - **Expected:** Erosion pattern converges to reference within tolerance; runs deterministically
     for same seed.

### TC-15.6.7.I1 Real-time navmesh regeneration preview

| # | Requirement |
|---|-------------|
| 1 | US-15.6.7   |

1. **#1** — Modify terrain and a placed obstacle; request navmesh regen.
   - **Expected:** Navmesh preview updates in under 1 s; generated polygons avoid obstacle and
     respect slope limits.

## Benchmarks

### TC-15.2.1.B1 Entity Placement Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Release-to-visible for single entity placement | Latency | < 1 ms | R-15.2.1 |

### TC-15.2.2.B1 Entity Template Propagation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Propagate source change to 1000 entity template instances | Duration | < 16 ms | R-15.2.2 |

### TC-15.2.4.B1 CSG Boolean Operation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Boolean union of 2 primitives (Box + Cylinder) | Duration | < 100 ms | R-15.2.4 |

### TC-15.6.1.B1 Terrain Sculpt Peak Memory

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sculpt across 16k x 16k heightmap | Peak memory | < 512 MB | R-15.6.1 |

### TC-15.6.2.B1 Erosion Preview

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hydraulic erosion preview on 2048x2048 region | Framerate | > 15 FPS | R-15.6.2 |

### TC-15.2.7.B1 Foliage Paint

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Paint stroke placing 10,000 foliage instances | Duration per stroke | < 32 ms | R-15.2.7 |

### TC-15.6.7.B1 Navmesh Regeneration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Regenerate navmesh for 64m x 64m region | Duration | < 500 ms | R-15.6.7 |
