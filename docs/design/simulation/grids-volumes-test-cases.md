# Grids and Volumes — Test Cases

Companion to [grids-volumes.md](grids-volumes.md). All test cases follow the `TC-X.Y.Z.N` format.
Requirements traced per the design Requirements Trace section.

## Unit Tests

| ID            | Test Name                                 | Req        |
|---------------|-------------------------------------------|------------|
| TC-13.20.1.1  | `test_grid_get_valid_cell`                | R-13.20.1  |
| TC-13.20.1.2  | `test_grid_get_out_of_bounds`             | R-13.20.1  |
| TC-13.20.1.3  | `test_grid_set_valid_cell`                | R-13.20.1  |
| TC-13.20.1.4  | `test_grid_set_edge_cell`                 | R-13.20.1  |
| TC-13.20.1.5  | `test_world_to_cell_inside`               | R-13.20.1  |
| TC-13.20.1.6  | `test_world_to_cell_outside`              | R-13.20.1  |
| TC-13.20.1.7  | `test_cell_to_world_corners`              | R-13.20.1  |
| TC-13.20.2.1  | `test_neighbors_cardinal_interior`        | R-13.20.2  |
| TC-13.20.2.2  | `test_neighbors_cardinal_edge`            | R-13.20.2  |
| TC-13.20.2.3  | `test_neighbors_all_corner`               | R-13.20.2  |
| TC-13.20.2.4  | `test_neighbors_all_center`               | R-13.20.2  |
| TC-13.20.2.5  | `test_los_clear_path`                     | R-13.20.2  |
| TC-13.20.2.6  | `test_los_blocked_cell`                   | R-13.20.2  |
| TC-13.20.3.1  | `test_flood_fill_open`                    | R-13.20.3  |
| TC-13.20.3.2  | `test_flood_fill_walled`                  | R-13.20.3  |
| TC-13.20.3.3  | `test_flood_fill_full_grid`               | R-13.20.3  |
| TC-13.20.4.1  | `test_area_query_within_radius`           | R-13.20.4  |
| TC-13.20.4.2  | `test_area_query_partial_overlap`         | R-13.20.4  |
| TC-13.21.1.1  | `test_propagate_decay`                    | R-13.21.1  |
| TC-13.21.1.2  | `test_propagate_max_radius`               | R-13.21.1  |
| TC-13.21.4.1  | `test_propagate_cardinal_pattern`         | R-13.21.4  |
| TC-13.21.4.2  | `test_propagate_diagonal_pattern`         | R-13.21.4  |
| TC-13.27.1.1  | `test_voxel_get_valid`                    | R-13.27.1  |
| TC-13.27.1.2  | `test_voxel_get_out_of_bounds`            | R-13.27.1  |
| TC-13.27.1.3  | `test_voxel_set_valid`                    | R-13.27.1  |
| TC-13.27.2.1  | `test_voxel_raycast_hit`                  | R-13.27.2  |
| TC-13.27.2.2  | `test_voxel_raycast_miss`                 | R-13.27.2  |
| TC-13.27.2.3  | `test_voxel_raycast_edge`                 | R-13.27.2  |
| TC-13.27.3.1  | `test_chunk_dirty_on_write`               | R-13.27.3  |
| TC-13.27.3.2  | `test_chunk_clear_dirty`                  | R-13.27.3  |
| TC-13.27.3.3  | `test_gpu_sync_merge_overlap`             | R-13.27.3  |
| TC-13.27.3.4  | `test_gpu_sync_merge_adjacent`            | R-13.27.3  |
| TC-7.6.8.1    | `test_cell_coord_equality`                | R-7.6.8    |
| TC-7.6.8.2    | `test_cell_coord_hash`                    | R-7.6.8    |
| TC-17.2.1.1   | `test_uniform_grid_primitive`             | R-17.2.1   |
| TC-17.2.2.1   | `test_propagate_256_budget`               | R-17.2.2   |
| TC-17.2.3.1   | `test_los_128_budget`                     | R-17.2.3   |
| TC-17.2.4.1   | `test_flow_field_dijkstra`                | R-17.2.4   |
| TC-17.2.5.1   | `test_influence_decay_block`              | R-17.2.5   |
| TC-17.2.6.1   | `test_voxel_volume_chunk_palette`         | R-17.2.6   |
| TC-17.2.7.1   | `test_voxel_raycast_512_budget`           | R-17.2.7   |
| TC-17.2.8.1   | `test_sdf_dirty_region_scope`             | R-17.2.8   |
| TC-17.2.9.1   | `test_gpu_dirty_upload_1ms`               | R-17.2.9   |
| TC-17.2.10.1  | `test_gpu_compute_propagation`            | R-17.2.10  |
| TC-17.2.11.1  | `test_hierarchical_grid_multires`         | R-17.2.11  |
| TC-17.2.12.1  | `test_aoi_grid_replication_filter`        | R-17.2.12  |

### Unit Test Details

1. **TC-13.20.1.1** -- Create 4×4 grid; get cell (1,1); verify returns default value.
2. **TC-13.20.1.2** -- Create 4×4 grid; get cell (4,4); verify returns `None`.
3. **TC-13.20.1.3** -- Create 4×4 grid; set cell (2,2) to value 7; get (2,2); verify returns 7.
4. **TC-13.20.1.4** -- Create 4×4 grid; set cell (0,3) (edge); verify correct storage and retrieval.
5. **TC-13.20.1.5** -- 8×8 grid, cell_size=1.0, origin=(0,0); world_to_cell((1.5, 2.5)); verify
   returns `Some(CellCoord { x:1, y:2 })`.
6. **TC-13.20.1.6** -- Same grid; world_to_cell((8.1, 0.0)); verify returns `None`.
7. **TC-13.20.1.7** -- 4×4 grid, cell_size=2.0, origin=(0,0); cell_to_world for all 4 corners;
   verify world centers are (1,1), (7,1), (1,7), (7,7).
8. **TC-13.20.2.1** -- Cardinal neighbors of interior cell (2,2) in 5×5 grid; verify 4 neighbors
   returned: (1,2), (3,2), (2,1), (2,3).
9. **TC-13.20.2.2** -- Cardinal neighbors of edge cell (0,2) in 5×5 grid; verify 3 neighbors
   (boundary excluded).
10. **TC-13.20.2.3** -- All neighbors of corner cell (0,0) in 5×5 grid; verify 3 neighbors.
11. **TC-13.20.2.4** -- All neighbors of center cell (2,2) in 5×5 grid; verify 8 neighbors.
12. **TC-13.20.2.5** -- 8×8 grid; LOS from (0,0) to (4,4) with no blocking cells; verify
    `clear=true`.
13. **TC-13.20.2.6** -- 8×8 grid; cell (2,2) is blocking; LOS from (0,0) to (4,4); verify
    `clear=false`, `blocked_at=Some(CellCoord{x:2,y:2})`.
14. **TC-13.20.3.1** -- 8×8 open grid; flood fill from (0,0); verify 64 cells returned.
15. **TC-13.20.3.2** -- 8×8 grid with wall across y=4; flood fill from (0,0); verify only 32 cells
    reached.
16. **TC-13.20.3.3** -- 4×4 grid, all passable; flood fill from (0,0); verify `cells_visited==16`.
17. **TC-13.20.4.1** -- 16×16 grid, cell_size=1.0; area_query center=(7.5, 7.5), radius=3.0; verify
    all returned cells have center within radius.
18. **TC-13.20.4.2** -- Same grid; area_query at corner center=(0.5,0.5), radius=3.0; verify no
    out-of-bounds cells returned.
19. **TC-13.21.1.1** -- 8×8 grid; single source cell at (4,4) with value 1.0; propagate with
    decay_rate=0.5; after 4 ticks verify value at (4,4) is < 0.1.
20. **TC-13.21.1.2** -- Same setup; max_radius=2; after propagation verify no cell beyond radius 2
    from source has non-zero value.
21. **TC-13.21.4.1** -- Propagate with `Cardinal` pattern; verify only 4-connected neighbors receive
    propagated values.
22. **TC-13.21.4.2** -- Propagate with `Diagonal` pattern; verify only diagonal neighbors receive
    propagated values.
23. **TC-13.27.1.1** -- Create 16×16×16 VoxelVolume; get voxel (8,8,8); verify returns default.
24. **TC-13.27.1.2** -- Same volume; get voxel (16,0,0); verify returns `None`.
25. **TC-13.27.1.3** -- Create volume; set voxel (5,5,5) to value 3; get (5,5,5); verify returns 3.
26. **TC-13.27.2.1** -- 16×16×16 volume; solid block at (8,8,8); raycast from (0,8,8) along +X;
    verify hit at coord (8,8,8).
27. **TC-13.27.2.2** -- Empty volume; raycast along any axis; verify returns `None`.
28. **TC-13.27.2.3** -- Volume; raycast that enters at edge voxel (0,0,0); verify correct hit coord
    and normal.
29. **TC-13.27.3.1** -- Create chunk; set a cell; verify `is_dirty()` returns `true`.
30. **TC-13.27.3.2** -- Create chunk; set a cell; call `clear_dirty()`; verify `is_dirty()` is
    `false`.
31. **TC-13.27.3.3** -- `GpuGridSync`; mark_dirty (0,0)-(2,2) then (1,1)-(3,3); drain; verify merged
    to single region (0,0)-(3,3).
32. **TC-13.27.3.4** -- Mark_dirty (0,0)-(1,1) then (2,2)-(3,3); drain; verify two separate regions
    returned.
33. **TC-7.6.8.1** -- `CellCoord { x:3, y:4 }` == `CellCoord { x:3, y:4 }`; verify true.
34. **TC-7.6.8.2** -- Two `CellCoord` with same values inserted into `HashSet`; verify set has size
    1.
35. **TC-17.2.1.1** -- Construct
    `UniformGrid<u8>::new(width=32, height=32, cell_size=1.0, origin= (0,0))`; read-back dimensions
    and cell_size; insert a value at (5,6); read it.
    - Input: `grid.set((5,6), 42)`; then `grid.get((5,6))`
    - Expected: `grid.width()==32`, `grid.height()==32`, `grid.get((5,6)) == Some(42)`
36. **TC-17.2.2.1** -- 256×256 `UniformGrid<f32>`; run one `PropagationKernel::propagate()` step
    with a single source cell. Measure wall time.
    - Input: 256×256 grid, single source, cardinal pattern, decay 0.9
    - Expected: single-step wall time < 1 ms
37. **TC-17.2.3.1** -- 256×256 grid; call `line_of_sight(start, end)` where the line length spans
    128 cells. Measure wall time.
    - Input: LOS from (0,0) to (127,0), no blocking cells
    - Expected: single-call wall time < 0.01 ms
38. **TC-17.2.4.1** -- Build a 2D flow field via Dijkstra propagation from a goal cell in a 32×32
    grid with a wall. Assert every reachable cell has a flow vector pointing one step closer (lower
    cost) to the goal.
    - Input: goal=(31,31), wall column at x=15
    - Expected: each non-wall cell's flow vector reduces Dijkstra cost toward goal
39. **TC-17.2.5.1** -- Influence propagation on a grid with a row of obstacle cells between the
    source and a target cell. Assert obstacles block propagation and decay reduces distant values.
    - Input: source at (0,0); obstacle row at y=4; propagate 10 steps
    - Expected: cells above y=4 have non-zero value; cells below y=4 have zero
40. **TC-17.2.6.1** -- Build `VoxelVolume<u8>` 64×64×64 with chunked storage + palette compression.
    Fill one chunk with a single value and another with two values.
    - Input: chunk A: all value 1; chunk B: half value 2, half value 3
    - Expected: chunk A's palette has 1 entry; chunk B's palette has 2 entries
41. **TC-17.2.7.1** -- 512×512×512 `VoxelVolume`; raycast along +X through 512 voxels. Measure wall
    time.
    - Input: solid block at (256, y, z); ray start=(0, 256, 256) dir=+X
    - Expected: single-call wall time < 2 ms; hit at voxel (256, 256, 256)
<!-- THIN: design section lacks detail -->
42. **TC-17.2.8.1** -- Modify a single voxel inside a `VoxelVolume` with an attached SDF. Recompute
    the SDF's dirty region and assert only cells within the affected bounding region are
    re-evaluated.
    - Input: 1-voxel mutation inside chunk (8,8,8)
    - Expected: SDF dirty region covers only the affected chunk's bounding neighborhood; other
      regions untouched
43. **TC-17.2.9.1** -- Mark 16 dirty 64×64 regions on a 1024×1024 grid; run `gpu_grid_sync_system`
    (staging + upload). Measure wall time.
    - Input: 16 dirty regions as above
    - Expected: end-to-end upload wall time < 1 ms
<!-- THIN: design section lacks detail -->
44. **TC-17.2.10.1** -- Trigger GPU compute propagation path when CPU budget exceeded: 1024×1024
    grid with a large propagation step. Assert the GPU compute dispatch is selected and CPU remains
    under its budget.
    - Input: 1024×1024 grid; CPU propagation > 2 ms threshold
    - Expected: GPU path dispatched; CPU propagation time < threshold
<!-- THIN: design section lacks detail -->
45. **TC-17.2.11.1** -- Build a `HierarchicalGrid` with two resolution levels (coarse 16×16 and fine
    256×256). Insert data at both levels and query across them. Assert fine-level writes propagate
    correctly to the coarse-level aggregate.
    - Input: set fine cells in one coarse region to value 1
    - Expected: coarse-level aggregate for that region reflects the fine data
46. **TC-17.2.12.1** -- Build an AOI `UniformGrid<EntitySet>` 64×64; insert 100 entities; query area
    around cell (32,32) with radius 5. Assert only entities in those cells are returned.
    - Input: 100 entities; area query at (32,32) radius 5
    - Expected: result equals brute-force enumeration of entities in the 5-cell-radius footprint

---

## Integration Tests

| ID            | Test Name                                     | Req        |
|---------------|-----------------------------------------------|------------|
| TC-13.20.1.8  | `test_grid_ecs_spawn_query_mutate`            | R-13.20.1  |
| TC-13.20.1.9  | `test_grid_spatial_index_lookup`              | R-13.20.1  |
| TC-13.21.1.3  | `test_propagation_multi_tick_spread`          | R-13.21.1  |
| TC-13.27.3.5  | `test_voxel_chunk_lifecycle`                  | R-13.27.3  |
| TC-13.27.1.4  | `test_grid_rkyv_round_trip`                   | R-13.27.1  |
| TC-13.20.1.10 | `test_gpu_sync_dirty_to_texture`              | R-13.20.1  |
| TC-7.6.8.3    | `test_scent_propagation_decay`                | R-7.6.8    |
| TC-13.21.4.2  | `test_tactical_cover_flanking_stance`         | R-13.21.4  |
| TC-13.20.2.7  | `test_visibility_culling_fog_grid`            | R-13.20.2  |
| TC-13.20.4.3  | `test_relevancy_grid_entities_in_radius`      | R-13.20.4  |
| TC-17.2.1.I1  | `test_author_uniform_grid`                    | US-17.2.1  |
| TC-17.2.2.I1  | `test_sim_propagate_256`                      | US-17.2.2  |
| TC-17.2.3.I1  | `test_sim_los_128`                            | US-17.2.3  |
| TC-17.2.4.I1  | `test_author_flow_field`                      | US-17.2.4  |
| TC-17.2.5.I1  | `test_author_influence_map`                   | US-17.2.5  |
| TC-17.2.6.I1  | `test_author_voxel_volume`                    | US-17.2.6  |
| TC-17.2.7.I1  | `test_sim_voxel_raycast_512`                  | US-17.2.7  |
| TC-17.2.8.I1  | `test_sim_sdf_dirty_region`                   | US-17.2.8  |
| TC-17.2.9.I1  | `test_sim_gpu_dirty_upload`                   | US-17.2.9  |
| TC-17.2.10.I1 | `test_sim_gpu_compute_prop`                   | US-17.2.10 |
| TC-17.2.11.I1 | `test_author_hierarchical_grid`               | US-17.2.11 |
| TC-17.2.12.I1 | `test_net_aoi_grid_filter`                    | US-17.2.12 |

### Integration Test Details

1. **TC-13.20.1.8** -- Spawn ECS entity with `UniformGrid<u8>` component; query it; set cell via
   mutable query; re-query and verify new value.
2. **TC-13.20.1.9** -- Grid with origin=(0,0), cell_size=1.0; insert 3 entities at known positions;
   `area_query` at center; verify all 3 returned.
3. **TC-13.21.1.3** -- 16×16 grid with `PropagationKernel<f32>`; advance 10 FixedUpdate ticks;
   verify value at distance 5 from source > 0.
4. **TC-13.27.3.5** -- Create `VoxelVolume`; set voxel in chunk; verify chunk marked dirty; clear
   dirty; verify clean; set again; verify dirty again.
5. **TC-13.27.1.4** -- Serialize a `UniformGrid<u8>` with rkyv; deserialize from byte slice; verify
   all 64 cells match original.
6. **TC-13.20.1.10** -- Grid with 3 dirty cells; attach `GpuGridSync`; run `gpu_grid_sync_system`;
   verify `GpuTextureManager` received correct region and data.
7. **TC-7.6.8.3** -- `UniformGrid<f32>` scent grid; inject value 1.0 at (4,4); run propagation
   kernel for 5 ticks with decay 0.8; verify cells at distance 3 have non-zero value and cells at
   max_radius have zero.
8. **TC-13.21.4.2** -- Tactical grid with cover values; entity queries cover at adjacent cells;
   flanking system reads occupancy; verify overwatch stance component updated.
9. **TC-13.20.2.7** -- Fog grid with half cells revealed; attach to GPU via `GpuGridSync`; run
   culling pass query; verify fogged entities excluded from render list.
10. **TC-13.20.4.3** -- `UniformGrid<EntitySet>` relevancy grid; insert 10 entities; call
    `entities_in_radius` with radius covering half the grid; verify correct subset returned.
11. **TC-17.2.1.I1** `test_author_uniform_grid` (US-17.2.1) — As a designer, create a
    `UniformGrid<u8>` 64×64 in the editor to author a tile hazard map; save and reload. Assert all
    64×64 cells round-trip.
    - Input: editor-authored grid with a checkerboard pattern
    - Expected: reloaded grid matches cell-for-cell
12. **TC-17.2.2.I1** `test_sim_propagate_256` (US-17.2.2) — As a simulation designer, run a 256×256
    influence propagation for one tick in the game loop. Assert the frame contribution meets the 1
    ms budget on reference hardware.
    - Input: 256×256 grid, one source, standard decay
    - Expected: measured tick contribution < 1 ms
13. **TC-17.2.3.I1** `test_sim_los_128` (US-17.2.3) — As an AI designer, query line of sight over
    128 cells in a gameplay scenario and assert the per-call budget of 0.01 ms holds.
    - Input: in-game LOS query at runtime
    - Expected: call latency < 0.01 ms
14. **TC-17.2.4.I1** `test_author_flow_field` (US-17.2.4) — As a designer, author a 2D flow field
    from a goal cell. Spawn an agent that follows the field. Assert the agent reaches the goal
    without bypassing walls.
    - Input: 32×32 grid, goal cell, one wall
    - Expected: agent path follows flow vectors; reaches goal
15. **TC-17.2.5.I1** `test_author_influence_map` (US-17.2.5) — As a strategy designer, author an
    influence map with decay and obstacle cells. Run propagation for multiple ticks; assert decay
    and obstacle blocking both take effect.
    - Input: influence source + obstacle row; 10 ticks
    - Expected: values past the obstacle are 0; values near source decayed per configured rate
16. **TC-17.2.6.I1** `test_author_voxel_volume` (US-17.2.6) — As a designer, author a
    `VoxelVolume<BlockId>` 64×64×64 in the editor using palette compression. Save and reload; assert
    palette and voxel data match.
    - Input: voxel volume with 3 distinct block IDs
    - Expected: round-trip yields identical palette size and cell values
17. **TC-17.2.7.I1** `test_sim_voxel_raycast_512` (US-17.2.7) — As a gameplay engineer, perform a
    runtime voxel raycast over 512 voxels in a gameplay scenario; assert the 2 ms budget holds.
    - Input: 512×512×512 volume, raycast along +X
    - Expected: call wall time < 2 ms
<!-- THIN: design section lacks detail -->
18. **TC-17.2.8.I1** `test_sim_sdf_dirty_region` (US-17.2.8) — As a designer, destroy a cluster of
    voxels at runtime; assert the SDF updates its dirty region and shadows/collision volumes near
    the cluster update within one frame.
    - Input: voxel mutation cluster
    - Expected: SDF dirty region covers the mutation; downstream systems observe updated SDF next
      frame
<!-- THIN: design section lacks detail -->
19. **TC-17.2.9.I1** `test_sim_gpu_dirty_upload` (US-17.2.9) — As a developer, mark 16 dirty grid
    regions during one frame; assert the GPU dirty upload completes within 1 ms as part of the
    frame's sync system.
    - Input: 16 dirty 64×64 regions
    - Expected: measured upload time < 1 ms
<!-- THIN: design section lacks detail -->
20. **TC-17.2.10.I1** `test_sim_gpu_compute_prop` (US-17.2.10) — As a developer, run a very large
    grid propagation where CPU budget is exceeded; assert the engine automatically switches to GPU
    compute propagation and the CPU frame budget is respected.
    - Input: 1024×1024 propagation with CPU timing over budget
    - Expected: GPU compute path taken; CPU frame time within budget
<!-- THIN: design section lacks detail -->
21. **TC-17.2.11.I1** `test_author_hierarchical_grid` (US-17.2.11) — As a world-sim designer, author
    a hierarchical multi-resolution grid with coarse and fine levels. Assert queries at the coarse
    level aggregate fine-level data correctly.
    - Input: 16×16 coarse + 256×256 fine
    - Expected: coarse cell (i,j) reflects aggregated fine cells underneath
22. **TC-17.2.12.I1** `test_net_aoi_grid_filter` (US-17.2.12) — As a netcode engineer, use an AOI
    uniform grid to filter replicated entities per client. Assert each client only receives entities
    within their relevancy radius.
    - Input: 200 entities, 4 clients each with 30-unit radius
    - Expected: replication snapshot per client contains only entities whose cell is within radius

---

## Benchmarks

| ID       | Test Name                           | Req          | Target     |
|----------|-------------------------------------|--------------|------------|
| TC-GV1.1 | `bench_propagate_256x256`           | NFR-SIM.GV1  | < 1 ms     |
| TC-GV2.1 | `bench_line_of_sight_128`           | NFR-SIM.GV2  | < 0.01 ms  |
| TC-GV3.1 | `bench_flood_fill_256x256_r64`      | NFR-SIM.GV3  | < 0.5 ms   |
| TC-GV4.1 | `bench_voxel_raycast_512`           | NFR-SIM.GV4  | < 2 ms     |
| TC-GV5.1 | `bench_gpu_sync_dirty_upload`       | NFR-SIM.GV5  | < 1 ms     |
| TC-GV6.1 | `bench_area_query_1024x1024_r32`    | NFR-SIM.GV1  | < 1 ms     |

### Benchmark Details

1. **TC-GV1.1** -- 256×256 `UniformGrid<f32>`; single source at center; `PropagationKernel` with
   Cardinal pattern, decay=0.9, max_radius=64; measure one `propagate()` call; target < 1 ms.
2. **TC-GV2.1** -- 128-cell `line_of_sight` call on a 256×256 grid; no blocking cells; measure call
   latency; target < 0.01 ms.
3. **TC-GV3.1** -- `flood_fill` on 256×256 grid, no walls, max radius=64; measure call latency;
   target < 0.5 ms.
4. **TC-GV4.1** -- `VoxelVolume` 512×512×512; `raycast` along max_dist=512 with sparse solids;
   measure call latency; target < 2 ms.
5. **TC-GV5.1** -- `GpuGridSync` with 16 dirty 64×64 regions on a 1024×1024 grid; measure
   `gpu_grid_sync_system` execution including staging upload; target < 1 ms.
6. **TC-GV6.1** -- `area_query` on 1024×1024 grid, radius=32 from center; measure call latency;
   target < 1 ms.
