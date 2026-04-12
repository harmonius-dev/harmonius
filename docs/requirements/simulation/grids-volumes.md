# R-17.2 — Grids and Volumes Requirements

## Uniform Grids

1. **R-17.2.1** — The engine **SHALL** provide a generic `UniformGrid<T>` primitive as a 2D grid of
   typed cells with fixed cell size and axis-aligned bounds, usable with Transform2D in 2D games and
   Transform in 3D games.
   - **Rationale:** One generic primitive replaces fog of war, tactical maps, height fields, scent
     grids, and influence maps — all 2D typed cell grids.
   - **Verification:** Unit test: build a 256x256 UniformGrid<u8>; write and read cells; assert
     correct values and out-of-bounds access returns None.
2. **R-17.2.2** — The engine **SHALL** compute 256x256 grid propagation (one step of
   PropagationKernel) within 1 ms per tick on desktop hardware.
   - **Rationale:** Propagation runs every tick for fire spread, fluid flow, and influence maps;
     frame budget demands tight bounds.
   - **Verification:** Benchmark: run one propagation step on a 256x256 UniformGrid; assert time
     under 1 ms.
3. **R-17.2.3** — The engine **SHALL** compute line-of-sight between two grid cells over a 128 cell
   distance within 0.01 ms.
   - **Rationale:** LOS queries run many times per tick for AI perception and fog reveal; per-query
     cost must stay well under 10 µs.
   - **Verification:** Benchmark: run 10,000 LOS queries on a 256x256 grid with mixed obstacles;
     assert mean time under 0.01 ms per query.
4. **R-17.2.4** — The engine **SHALL** compute 2D flow fields from goal positions via Dijkstra
   propagation on uniform grids, producing per-cell direction vectors for mass agent navigation.
   - **Rationale:** RTS unit movement (hundreds of units toward one goal) demands flow fields;
     per-agent A* does not scale.
   - **Verification:** Unit test: compute a flow field on a 64x64 grid with one goal; step an agent
     from any cell along the flow; assert reaches goal monotonically.
5. **R-17.2.5** — The engine **SHALL** propagate influence values from source entities across grid
   cells with configurable decay rate per step and obstacle blocking.
   - **Rationale:** Influence maps drive AI decision making (threat zones, resource attraction) and
     must support decay and obstacle semantics.
   - **Verification:** Unit test: place a source; propagate 10 steps with decay 0.9; assert
     influence at distance k equals 0.9^k within tolerance.

## Voxel Volumes

1. **R-17.2.6** — The engine **SHALL** provide a `VoxelVolume<T>` primitive as a 3D grid of typed
   cells stored in chunks with palette compression for blocks, used only for 3D games.
   - **Rationale:** Block worlds, density fields, and 3D wind volumes all share chunk-based voxel
     storage; palette compression keeps memory bounded.
   - **Verification:** Unit test: build a 64x64x64 VoxelVolume; fill with palette-repeating blocks;
     assert per-chunk palette count matches and memory usage matches palette math.
2. **R-17.2.7** — The engine **SHALL** raycast a ray through a voxel volume of up to 512 voxels in
   length within 2 ms on desktop hardware, returning the first non-empty voxel hit.
   - **Rationale:** Block placement, destruction, and shooting rays need fast voxel traversal at
     interactive rates.
   - **Verification:** Benchmark: run 1,000 raycasts on a 256^3 voxel volume; assert mean under 2 ms
     per 512-voxel ray.
3. **R-17.2.8** — The engine **SHALL** compute signed distance fields from voxel volume surfaces for
   soft shadows, ambient occlusion, and collision queries, with SDF generation scoped to dirty
   regions.
   - **Rationale:** SDF is a reusable spatial representation that many rendering and physics
     features consume; recomputing on dirty regions only keeps cost bounded.
   - **Verification:** Unit test: author a simple voxel sphere; compute SDF; assert known sampled
     points match analytic SDF within tolerance.

## GPU Sync

1. **R-17.2.9** — The engine **SHALL** upload uniform grid dirty regions to the GPU within 1 ms per
   tick so that renderer overlays reflect simulation state with at most one frame of lag.
   - **Rationale:** Fog of war, heat maps, and wind visualization render from grid textures; upload
     cost must fit in the per-frame budget.
   - **Verification:** Benchmark: mark a 64x64 dirty region; upload to GPU; assert total time under
     1 ms.
2. **R-17.2.10** — The engine **SHALL** support GPU compute shader execution for large-scale grid
   propagation (fire spread, fluid flow) when CPU propagation exceeds the configured per-tick
   budget.
   - **Rationale:** Very large grids (1024x1024+) exceed CPU budget; GPU compute offloads the
     propagation stage where hardware allows.
   - **Verification:** Integration test: configure CPU budget 1 ms; run propagation on 1024^2 grid;
     assert automatic GPU offload triggered and results match CPU baseline.

## Hierarchical Grids

1. **R-17.2.11** — The engine **SHALL** provide a hierarchical grid variant with multi-resolution
   cells for large worlds requiring variable detail, where coarse cells subdivide into finer cells
   on demand.
   - **Rationale:** Open-world 2D games need detail near the camera and coarse state far away;
     uniform grids force one resolution everywhere.
   - **Verification:** Unit test: build a hierarchical grid; subdivide a coarse cell; assert queries
     at both levels return consistent values.

## Area of Interest

1. **R-17.2.12** — The engine **SHALL** maintain a uniform grid of entity sets for networking
   area-of-interest filtering, where the replication system queries nearby cells to determine
   relevancy for a given observer.
   - **Rationale:** Networking must filter replication by proximity; a dedicated AOI grid avoids
     global BVH queries on every observer per tick.
   - **Verification:** Integration test: populate a grid with 1,000 entities; query AOI radius 10
     cells around an observer; assert result set matches brute-force expectation.
