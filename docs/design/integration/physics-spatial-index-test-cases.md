# Physics ↔ Spatial Index Integration Test Cases

All test cases are CI-runnable under `cargo test` with no external services or GPU required.
Coverage: 5 Integration Requirements, 11 unit tests, 14 integration tests, 8 negative tests, and 5
benchmarks.

## Unit Tests

| ID | Name | Req |
|----|------|-----|
| TC-IR-3.9.1.U1 | Overlap two AABBs | IR-3.9.1 |
| TC-IR-3.9.1.U2 | Overlap disjoint AABBs | IR-3.9.1 |
| TC-IR-3.9.2.U1 | PhysicsBvh is private Res | IR-3.9.2 |
| TC-IR-3.9.3.U1 | Layer interacts_with pass | IR-3.9.3 |
| TC-IR-3.9.3.U2 | Layer interacts_with fail | IR-3.9.3 |
| TC-IR-3.9.3.U3 | Layer zero on both sides | IR-3.9.3 |
| TC-IR-3.9.4.U1 | BatchSpatialQuery dispatch | IR-3.9.4 |
| TC-IR-3.9.4.U2 | MPSC buffer cap 1024 | IR-3.9.4 |
| TC-IR-3.9.5.U1 | Fat AABB velocity expand | IR-3.9.5 |
| TC-IR-3.9.5.U2 | BvhHandle generation bump | IR-3.9.5 |
| TC-IR-3.9.5.U3 | Refit runs once per frame | IR-3.9.5 |

### Unit Test Details

1. **TC-IR-3.9.1.U1** -- Input: two unit cubes at origin and at (0.5, 0, 0). Expected: AABB overlap
   returns `true`.
2. **TC-IR-3.9.1.U2** -- Input: cubes at origin and at (100, 0, 0). Expected: AABB overlap returns
   `false`.
3. **TC-IR-3.9.2.U1** -- Input: AI system declares `Res<PhysicsBvh>`. Expected: compile error via
   sealed marker or visibility check; AI must not see the physics-private resource type.
4. **TC-IR-3.9.3.U1** -- Input: `CollisionLayers { m=0x01, mk=0x02 }` vs
   `CollisionLayers { m=0x02, mk=0x01 }`. Expected: `interacts_with` returns `true`.
5. **TC-IR-3.9.3.U2** -- Input: `{ m=0x01, mk=0x01 }` vs `{ m=0x02, mk=0x02 }`. Expected:
   `interacts_with` returns `false`.
6. **TC-IR-3.9.3.U3** -- Input: two entities with `{ m=0, mk=0 }`. Expected: `interacts_with`
   returns `false`; one warning emitted per entity per session.
7. **TC-IR-3.9.4.U1** -- Input: batch with 4 rays + 4 shape casts. Expected: 8 results returned;
   assertion confirms every task ran on a worker-pool thread (not main, not render).
8. **TC-IR-3.9.4.U2** -- Input: 1024 queued batch results saturating the MPSC channel. Expected: no
   drops, no deadlock, all results received.
9. **TC-IR-3.9.5.U1** -- Input: entity with velocity 5 m/s and `dt_frame = 1/60`. Expected: leaf
   AABB inflated by `5/60 + margin` in the velocity direction.
10. **TC-IR-3.9.5.U2** -- Input: entity despawned and respawned, reusing the slot. Expected: prior
    `BvhHandle` rejected by generational check on refit.
11. **TC-IR-3.9.5.U3** -- Input: physics frame with N=4 substeps. Expected:
    `PhysicsBvhUpdateSystem::refit_count == 1` for the frame.

## Integration Tests

| ID | Name | Req |
|----|------|-----|
| TC-IR-3.9.1.1 | Broadphase finds overlap | IR-3.9.1 |
| TC-IR-3.9.1.2 | Broadphase rejects far pair | IR-3.9.1 |
| TC-IR-3.9.1.3 | Broadphase feeds narrowphase | IR-3.9.1 |
| TC-IR-3.9.2.1 | Shared BvhIndex untouched | IR-3.9.2 |
| TC-IR-3.9.2.2 | AI reads shared not physics | IR-3.9.2 |
| TC-IR-3.9.3.1 | Layer filter passes pair | IR-3.9.3 |
| TC-IR-3.9.3.2 | Layer filter rejects pair | IR-3.9.3 |
| TC-IR-3.9.3.3 | Trigger volume uses layers | IR-3.9.3 |
| TC-IR-3.9.4.1 | Batch ray cast parallel | IR-3.9.4 |
| TC-IR-3.9.4.2 | Batch shape cast for CCD | IR-3.9.4 |
| TC-IR-3.9.4.3 | Batch scales with threads | IR-3.9.4 |
| TC-IR-3.9.5.1 | Refit after move | IR-3.9.5 |
| TC-IR-3.9.5.2 | Static skips refit | IR-3.9.5 |
| TC-IR-3.9.5.3 | Substep uses fat AABBs | IR-3.9.5 |

### Integration Test Details

1. **TC-IR-3.9.1.1** -- Input: two overlapping box colliders. Expected: broadphase emits a
   `BroadphasePair` sourced from `PhysicsBvh`.
2. **TC-IR-3.9.1.2** -- Input: box colliders 100 m apart. Expected: no broadphase pair.
3. **TC-IR-3.9.1.3** -- Input: overlapping pair fed to narrowphase. Expected: `ContactManifold`
   generated with valid contacts.
4. **TC-IR-3.9.2.1** -- Input: run physics for 1 s of simulated time. Expected: shared `BvhIndex`
   revision counter is unchanged (physics never touched it).
5. **TC-IR-3.9.2.2** -- Input: AI spatial query. Expected: query hits shared `BvhIndex` only;
   instrumentation confirms `PhysicsBvh` was not accessed.
6. **TC-IR-3.9.3.1** -- Input: compatible membership/mask pair. Expected: broadphase emits pair.
7. **TC-IR-3.9.3.2** -- Input: incompatible membership/mask pair. Expected: no pair emitted.
8. **TC-IR-3.9.3.3** -- Input: trigger volume and player sharing a layer. Expected: `TriggerEnter`
   event fired on overlap.
9. **TC-IR-3.9.4.1** -- Input: 100 rays from the same origin in varied directions. Expected: all 100
   results returned; every task scheduled on a worker thread.
10. **TC-IR-3.9.4.2** -- Input: 50 swept-sphere CCD queries. Expected: TOI value per query.
11. **TC-IR-3.9.4.3** -- Input: 1000 batch queries run with 4-thread and 8-thread pool sizes.
    Expected: near-linear speedup (within 15% of ideal).
12. **TC-IR-3.9.5.1** -- Input: 100 dynamic bodies translated 1 m. Expected: corresponding leaf
    AABBs updated in the next frame.
13. **TC-IR-3.9.5.2** -- Input: 50 static bodies, no changes. Expected: zero refit operations
    counted.
14. **TC-IR-3.9.5.3** -- Input: 4 substeps with high-velocity bodies. Expected: no missed broadphase
    pair thanks to fat AABB expansion; narrowphase produces contacts against live transforms.

## Negative Tests

| ID | Name | Req |
|----|------|-----|
| TC-IR-3.9.1.N1 | Degenerate BVH query | IR-3.9.1 |
| TC-IR-3.9.1.N2 | Missed refit detection | IR-3.9.1 |
| TC-IR-3.9.3.N1 | Layer mask zero warning | IR-3.9.3 |
| TC-IR-3.9.3.N2 | Construction assert zero | IR-3.9.3 |
| TC-IR-3.9.4.N1 | Sweep distance cap | IR-3.9.4 |
| TC-IR-3.9.4.N2 | Pair explosion | IR-3.9.4 |
| TC-IR-3.9.5.N1 | Stale BvhHandle refit | IR-3.9.5 |
| TC-IR-3.9.5.N2 | Despawn mid-query | IR-3.9.5 |

### Negative Test Details

1. **TC-IR-3.9.1.N1** -- Input: 10 000 colinear AABBs forcing degenerate topology. Expected:
   background full rebuild triggered, info log emitted, queries still succeed.
2. **TC-IR-3.9.1.N2** -- Input: dirty flag set on more than 10% of leaves. Expected: forced full
   refit before broadphase, warning logged with frame index.
3. **TC-IR-3.9.3.N1** -- Input: entity with `CollisionLayers { m=0, mk=0 }`. Expected: broadphase
   emits no pair for it; one warning per entity per session.
4. **TC-IR-3.9.3.N2** -- Input: `CollisionLayers::new(0, 0)`. Expected: construction-time assertion
   panics with a message naming the invalid configuration.
5. **TC-IR-3.9.4.N1** -- Input: swept ray of 200 m with 64 m cap. Expected: truncated at 64 m,
   warning logged with the querying entity.
6. **TC-IR-3.9.4.N2** -- Input: scene producing `BroadphasePairs.len() > 4 * entity_count`.
   Expected: island culling and sleep triggered, warning logged with pair count.
7. **TC-IR-3.9.5.N1** -- Input: `BvhHandle` whose generation was bumped by a prior despawn.
   Expected: refit rejects the handle; leaf untouched; no panic.
8. **TC-IR-3.9.5.N2** -- Input: entity despawned during an in-flight batch query. Expected: no stale
   pair reported, no panic; generational handle rejects the read.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.9.1.B1 | Broadphase 50K AABBs | < 1 ms (R-4.2.NF1) | IR-3.9.1 |
| TC-IR-3.9.4.B1 | Ray cast 1M entities | < 10 us/ray (R-1.9.4a) | IR-3.9.4 |
| TC-IR-3.9.4.B2 | Batch 1024 shape casts | < 2 ms aggregate | IR-3.9.4 |
| TC-IR-3.9.5.B1 | Refit 1000 moved entities | < 0.5 ms | IR-3.9.5 |
| TC-IR-3.9.5.B2 | BVH memory per entity | <= 64 bytes (R-1.9.1a) | IR-3.9.5 |

Benchmarks use `criterion` and run under `cargo bench`. They are also gated behind a `bench-smoke`
`cargo test` feature that runs them with reduced iteration counts so CI can catch gross regressions
without the full benchmark runtime.
