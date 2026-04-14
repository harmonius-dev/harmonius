# Grids/Volumes ↔ Physics Integration Test Cases

All tests are CI-runnable via `cargo test` (unit + integration) and `cargo bench` (benchmarks). No
mocking; tests use real `VoxelVolume`, real `UniformGrid`, and real `PhysicsQueries` with a minimal
in-memory `PhysicsWorld` fake that implements the production `PhysicsEngine` trait.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.10.U1 | Request rkyv round-trip | Sphere r=3 (8,8,8) | Archive == orig | IR-3.10.3 |
| TC-IR-3.10.U2 | Enum complete | All variants | 3 variants | IR-3.10.3 |
| TC-IR-3.10.U3 | Inline 4 entities | Push 4 | No heap alloc | IR-3.10.4 |
| TC-IR-3.10.U4 | Spill to heap | Push 5 | Correct len=5 | IR-3.10.4 |
| TC-IR-3.10.U5 | LOS zero-distance | from == to | Returns true | IR-3.10.5 |
| TC-IR-3.10.U6 | LOS cache hit | Insert then get | Cached value | IR-3.10.5 |
| TC-IR-3.10.U7 | LOS cache miss | Empty cache | None | IR-3.10.5 |
| TC-IR-3.10.5.2 | Open LOS propagation | +x, empty BVH | `true` | IR-3.10.5 |

1. **TC-IR-3.10.U1** -- Round-trip a `VoxelDestructionRequest { Sphere, radius 3, coord (8,8,8) }`
   through `rkyv::Archive`/`Deserialize`. Verify bitwise equality to the original.
2. **TC-IR-3.10.U2** -- Exhaustive match on `DestructionPattern` compiles and covers `Sphere`,
   `Cone`, `Column` with no wildcard arm.
3. **TC-IR-3.10.U3** -- Push 4 `Entity` values into `OccupancyUpdate.entities`. Assert
   `entities.spilled() == false` (inline storage).
4. **TC-IR-3.10.U4** -- Push 5 `Entity` values. Assert `entities.len() == 5` and
   `entities.spilled() == true`.
5. **TC-IR-3.10.U5** -- Call `propagation_los_check(p, p, ...)`. Expect early-out `true`.
6. **TC-IR-3.10.U6** -- Insert `(a, b) -> true` into `LosCache`. Assert `get(a, b) == Some(true)`.
7. **TC-IR-3.10.U7** -- Query empty `LosCache`. Assert `get(a, b) == None`.
8. **TC-IR-3.10.5.2** -- `tests/tc_ir_3_10_5_2_propagation.rs` exercises `propagation_los_check`
   with a `PhysicsQueries` fake and empty BVH. Full fire-kernel acceptance stays deferred to later
   integration rows.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.10.1.1 | Height grid feeds collider | IR-3.10.1 |
| TC-IR-3.10.1.2 | Grid edit updates collider | IR-3.10.1 |
| TC-IR-3.10.1.3 | Cell-to-world maps correctly | IR-3.10.1 |
| TC-IR-3.10.2.1 | Voxel chunk generates collider | IR-3.10.2 |
| TC-IR-3.10.2.2 | Voxel place rebuilds collider | IR-3.10.2 |
| TC-IR-3.10.2.3 | Voxel remove rebuilds collider | IR-3.10.2 |
| TC-IR-3.10.3.1 | Destruction clears voxels | IR-3.10.3 |
| TC-IR-3.10.3.2 | Destruction triggers remesh | IR-3.10.3 |
| TC-IR-3.10.3.3 | Destruction updates collision | IR-3.10.3 |
| TC-IR-3.10.3.4 | One-tick latency honored | IR-3.10.3 |
| TC-IR-3.10.4.1 | Occupancy tracks body position | IR-3.10.4 |
| TC-IR-3.10.4.2 | Occupancy clears on body move | IR-3.10.4 |
| TC-IR-3.10.4.3 | Multiple bodies in one cell | IR-3.10.4 |
| TC-IR-3.10.4.4 | Five bodies in one cell | IR-3.10.4 |
| TC-IR-3.10.5.1 | Propagation blocked by wall | IR-3.10.5 |
| TC-IR-3.10.5.3 | LOS ray uses physics-private BVH | IR-3.10.5 |
| TC-IR-3.10.5.4 | LOS cache amortizes cost | IR-3.10.5 |

1. **TC-IR-3.10.1.1** -- 64x64 `UniformGrid<HeightCell>` with peak value at cell (32,32). Build a
   `ColliderShape::Heightfield` from it. Assert the collider's sampled height at (32,32) equals the
   grid cell value.
2. **TC-IR-3.10.1.2** -- Raise cell (10,10) by 5.0, trigger rebuild. Assert collider's height at
   (10,10) is 5.0 higher than baseline.
3. **TC-IR-3.10.1.3** -- Grid with `cell_size=2.0` and `origin=(10,10)`. Assert
   `cell_to_world(CellCoord(0,0)) == (10.0, 10.0)`.
4. **TC-IR-3.10.2.1** -- 16^3 `VoxelVolume<BlockType>`, lower half solid. Mesh to
   `ColliderShape::TriangleMesh`. Assert mesh is non-empty and all normals have unit length.
5. **TC-IR-3.10.2.2** -- Place a block at (8,8,8); rebuild collider. Assert mesh triangles include
   faces at the new block's AABB.
6. **TC-IR-3.10.2.3** -- Remove a previously solid block at (8,8,8); rebuild. Assert no mesh
   triangles inside the former block's AABB.
7. **TC-IR-3.10.3.1** -- `VoxelDestructionRequest { Sphere, r=3, (8,8,8) }` applied to a solid
   volume. Assert every voxel within Euclidean distance 3 of (8,8,8) is `BlockType::Air`.
8. **TC-IR-3.10.3.2** -- Blast in chunk (0,0,0). Assert the chunk's `is_dirty()` flag is set and the
   remesher processes it on the next simulation tick.
9. **TC-IR-3.10.3.3** -- Remove a horizontal floor of voxels under a dynamic rigid body. Step
   physics one fixed tick after rebuild. Assert body's y-position decreased under gravity.
10. **TC-IR-3.10.3.4** -- Emit `DestructionEvent` at tick N. Assert the updated collider is live in
    the physics-private BVH by the start of Phase 5 at tick N+1 (never at tick N).
11. **TC-IR-3.10.4.1** -- Place a rigid body at world (5.0, 0.0) on a grid with `cell_size=2.0`. Run
    occupancy. Assert cell (2,0) is marked occupied with the body's `Entity`.
12. **TC-IR-3.10.4.2** -- Move body from (5,0) to (15,0). Assert cell (2,0) is now clear and cell
    (7,0) is occupied.
13. **TC-IR-3.10.4.3** -- Two bodies overlap cell (3,3). Assert `OccupancyUpdate.entities` length is
    2 and contains both.
14. **TC-IR-3.10.4.4** -- Five bodies overlap cell (3,3). Assert length is 5 and the `SmallVec` has
    spilled to heap.
15. **TC-IR-3.10.5.1** -- Fire propagation kernel with a wall collider between cells A and B. Assert
    cell B's fire value stays zero after N ticks.
16. **TC-IR-3.10.5.3** -- Fire an LOS raycast; assert the resulting `RayHit.entity` matches the wall
    entity inserted into the physics-private BVH. Confirm the shared BVH is never consulted.
17. **TC-IR-3.10.5.4** -- Query the same cell pair twice in one tick. Assert the second call is
    served from `LosCache` (instrument a counter on the physics `ray_cast` path).

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.10.N1 | Impact outside volume | IR-3.10.3 |
| TC-IR-3.10.N2 | Destruction on unloaded chunk | IR-3.10.3 |
| TC-IR-3.10.N3 | MPSC destruction overflow | IR-3.10.3 |
| TC-IR-3.10.N4 | Occupancy desync recovery | IR-3.10.4 |
| TC-IR-3.10.N5 | PhysicsQueries uninitialized | IR-3.10.5 |
| TC-IR-3.10.N6 | Remesh budget exceeded | IR-3.10.2 |
| TC-IR-3.10.N7 | Blocking I/O forbidden | IR-3.10.3 |
| TC-IR-3.10.N8 | Cone zero half_angle | IR-3.10.3 |
| TC-IR-3.10.N9 | Sphere radius zero | IR-3.10.3 |
| TC-IR-3.10.N10 | Heightfield OOB cell | IR-3.10.1 |

1. **TC-IR-3.10.N1** -- Blast at world (-100,-100,-100) against a volume at origin. Assert volume is
   unchanged and no panic occurs.
2. **TC-IR-3.10.N2** -- Request destruction in a chunk whose residency handle is pending. Assert the
   request is dropped and a warning is logged; no crash.
3. **TC-IR-3.10.N3** -- Push 257 `VoxelDestructionRequest` into a capacity-256 MPSC. Assert the
   oldest entry is dropped, a warning is logged, and the newest 256 remain.
4. **TC-IR-3.10.N4** -- Artificially skew occupancy state vs physics body positions. Run the
   every-N-tick full rebuild. Assert occupancy matches body positions afterwards.
5. **TC-IR-3.10.N5** -- Invoke propagation LOS before `PhysicsQueries` is populated (frame 0).
   Assert the kernel spreads freely and no panic occurs.
6. **TC-IR-3.10.N6** -- Mark 20 chunks dirty on desktop (budget 8). Assert 8 are remeshed this
   frame, 12 queued for the next, and no remesh is dropped.
7. **TC-IR-3.10.N7** -- Exercise the "chunk unloaded" path while instrumenting `read`, `pread`, and
   `fread`. Assert zero synchronous read syscalls occur on the hot path.
8. **TC-IR-3.10.N8** -- `VoxelDestructionRequest { Cone { half_angle: 0.0 }, ... }`. Assert only the
   single impact voxel is cleared and no panic on zero-angle cone.
9. **TC-IR-3.10.N9** -- `VoxelDestructionRequest { Sphere, radius: 0, ... }`. Assert no voxels are
   cleared.
10. **TC-IR-3.10.N10** -- Call `world_to_cell` with a far-out world position yielding out-of- bounds
    coordinates. Assert `None` is returned and no panic occurs.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.10.1.B1 | Heightfield rebuild 257x257 | < 5 ms CPU | IR-3.10.1 |
| TC-IR-3.10.2.B1 | Voxel remesh 16^3 chunk | < 5 ms (NFR-SIM.GV4) | IR-3.10.2 |
| TC-IR-3.10.3.B1 | Destruction blast r=5 | < 1 ms CPU | IR-3.10.3 |
| TC-IR-3.10.3.B2 | MPSC push 1k requests | < 0.2 ms CPU | IR-3.10.3 |
| TC-IR-3.10.4.B1 | Occupancy 1000 bodies | < 0.5 ms CPU | IR-3.10.4 |
| TC-IR-3.10.4.B2 | Occupancy 4+ per cell | < 0.6 ms CPU | IR-3.10.4 |
| TC-IR-3.10.5.B1 | Propagation 256x256 LOS | < 1 ms (NFR-SIM.GV1) | IR-3.10.5 |
| TC-IR-3.10.5.B2 | LOS cache hit rate | > 80% on fire kernel | IR-3.10.5 |
