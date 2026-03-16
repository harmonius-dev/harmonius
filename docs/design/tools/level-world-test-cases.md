# Level Editor and World Building Test Cases

Companion test cases for [level-world.md](level-world.md).

## Unit Tests

### TC-15.2.1.1 Grid Snap Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Position `(1.3, 0, 2.7)`, `SnapMode::Grid { cell_size: 1.0 }` | Snapped position `(1.0, 0.0, 3.0)` | R-15.2.1 |
| 2 | Position `(2.5, 0, 4.5)`, `SnapMode::Grid { cell_size: 0.5 }` | Snapped position `(2.5, 0.0, 4.5)` | R-15.2.1 |
| 3 | Position `(0.1, 0, 0.1)`, `SnapMode::Grid { cell_size: 1.0 }` | Snapped position `(0.0, 0.0, 0.0)` | R-15.2.1 |

### TC-15.2.1.2 Surface Snap Terrain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raycast onto terrain slope at 30 degrees | Entity aligns to surface normal (rotation ~30 degrees from vertical) | R-15.2.1 |

### TC-15.2.1.3 Vertex Snap Precision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Snap to vertex at `[5.0, 3.0, 7.0]` | Snapped position within epsilon `1e-5` of `[5.0, 3.0, 7.0]` | R-15.2.1 |

### TC-15.2.2.1 Prefab Instantiate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instantiate 3-level nested prefab (root -> child -> grandchild) | All 3 entities spawned with correct parent-child hierarchy | R-15.2.2 |

### TC-15.2.2.2 Prefab Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify source prefab `hp: 100 -> 200`, instance without override | Instance `hp == 200` | R-15.2.2 |
| 2 | Modify source prefab `hp: 100 -> 200`, instance with `hp` overridden to 50 | Instance `hp == 50` (override preserved) | R-15.2.2 |

### TC-15.2.3.1 Override Set Revert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `override.set(path, value_B)` where source is value_A | `has_override(path) == true`, entity has value_B | R-15.2.3 |
| 2 | `revert_property(path)` | `has_override(path) == false`, entity has value_A | R-15.2.3 |

### TC-15.2.3.2 Override Apply to Source

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Override `hp = 150`, `apply_to_source(path)` | Source prefab `hp == 150`, all non-overridden instances update to 150 | R-15.2.3 |

### TC-15.2.4.1 CSG Boolean Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two `Box { extents: [1,1,1] }` brushes overlapping, `boolean_op(a, b)` | Result is watertight mesh, no degenerate triangles | R-15.2.4 |

### TC-15.2.4.2 CSG Boolean Subtractive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Box { extents: [2,2,2] }` minus `Cylinder { radius: 0.5, height: 3.0 }` | Result mesh has cylindrical hole, watertight | R-15.2.4 |

### TC-15.2.4.3 CSG to Static Mesh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `convert_to_static_mesh(brush)` | Returns valid `AssetId`, mesh asset registered | R-15.2.4 |

### TC-15.2.5.1 Spline Bezier C1

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create Bezier spline with 4 control points | First derivative continuous at all control points (C1 continuity) | R-15.2.5 |

### TC-15.2.5.2 Spline Distribute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `distribute_along(spline, DistributeConfig { spacing: 1.0, .. })`, spline length 10 | 10 entities placed at ~1.0 unit intervals | R-15.2.5 |

### TC-15.2.6.1 Landscape Auto Paint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rule: layer 0 for slope 0-30, layer 1 for slope 30-60 | Terrain at 20 degrees has layer 0, terrain at 45 degrees has layer 1 | R-15.2.6 |

### TC-15.2.6.2 Landscape Weight Sum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Paint layer 0 at 0.6, paint layer 1 at 0.4, check weights | Weights sum to 1.0 per texel (within epsilon) | R-15.2.6 |

### TC-15.2.7.1 Foliage Slope Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `FoliagePlacementRule { max_slope: 30.0, .. }`, terrain has 45-degree region | No foliage instances placed on 45-degree slopes | R-15.2.7 |

### TC-15.2.7.2 Foliage Exclusion Zone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_exclusion_zone(bounds)`, paint foliage over zone | No foliage instances within exclusion bounds | R-15.2.7 |

### TC-15.6.1.1 Sculpt Raise Lower

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply `SculptBrushType::Raise` at center | Heightmap values at brush center increased | R-15.6.1 |
| 2 | Apply `SculptBrushType::Lower` at same center | Heightmap values decreased below raised level | R-15.6.1 |

### TC-15.6.1.2 Sculpt Streaming

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sculpt across 16k x 16k heightmap | Peak memory usage < 512 MB | R-15.6.1 |

### TC-15.6.2.1 Erosion Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `simulate(region, Hydraulic, params)` twice with same params | Identical heightmap output both runs | R-15.6.2 |

### TC-15.6.4.1 Water River Spline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create_river(spline, RiverConfig { width: 5.0, depth: 2.0, .. })` | Entity spawned with water components, following spline path | R-15.6.4 |

### TC-15.6.4.2 Water Lake Altitude

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create_lake(boundary, LakeConfig { surface_altitude: 100.0, .. })` | Water surface entity at altitude 100.0 | R-15.6.4 |

### TC-15.6.5.1 Biome Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `auto_populate(region, rules, seed: 42)` twice | Same entity count and positions both runs | R-15.6.5 |

### TC-15.6.5.2 Biome Rule Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `auto_populate(region, rules)`, `validate_placements(region, rules)` | Returns empty violations list | R-15.6.5 |

### TC-15.6.6.1 Probe Grid Placement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `place_light_probe_grid(bounds, TetrahedralGrid { spacing: 5.0 })` in 20x20x20 area | Probes placed at ~5 unit spacing, count approximately 64 | R-15.6.6 |

### TC-15.6.7.1 Navmesh Slope Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate navmesh, terrain has 60-degree slope region | Steep region marked non-walkable in navmesh | R-15.6.7 |

### TC-15.6.8.1 Partition Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cell with `entity_budget: 100`, add 150 entities | `CellBudgetStatus { over_budget: true, entity_count: 150 }` | R-15.6.8 |

### TC-15.6.8.2 Multiuser Lock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A `request_lock(cell(3,7))` | Returns `Ok(CellLock)` | R-15.6.8 |
| 2 | User B `request_lock(cell(3,7))` while A holds lock | Returns `Err(LockError::CellOwnedBy(user_A))` | R-15.6.8 |

## Integration Tests

### TC-15.2.1.I1 Place Undo Redo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place entity via EntityPlacer, undo | Entity removed from ECS world | R-15.2.1 |
| 2 | Redo after undo | Entity restored at same position | R-15.2.1 |

### TC-15.2.2.I1 Prefab Save Load Round Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create nested prefab, save to disk, load | Loaded prefab structure identical to saved | R-15.2.2 |

### TC-15.2.4.I1 CSG Render Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | CSG union of two boxes, render and generate collision | Visual mesh renders correctly, collision mesh matches visual | R-15.2.4 |

### TC-15.6.1.I1 Terrain Sculpt Async IO

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sculpt terrain with async I/O enabled | No worker threads blocked during sculpting, UI remains responsive | R-15.6.1 |

### TC-15.6.2.I1 Erosion GPU 15fps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run erosion preview on 2048x2048 terrain region | Preview framerate stays above 15 FPS | R-15.6.2 |

### TC-15.6.8.I1 Multiuser Concurrent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A edits cell (2,3), user B edits cell (5,7) concurrently | Both changes persist after sync | R-15.6.8 |

### TC-15.6.8.I2 Partition Streaming

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load cell (1,1), unload cell (0,0) | Cell (1,1) state is `Loaded`, cell (0,0) state is `Unloaded` in overlay | R-15.6.8 |

## Benchmarks

### TC-15.2.1.B1 Entity Placement Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Release-to-visible for single entity placement | Latency | < 1 ms | R-15.2.1 |

### TC-15.2.2.B1 Prefab Propagation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Propagate source change to 1000 prefab instances | Duration | < 16 ms | R-15.2.2 |

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
