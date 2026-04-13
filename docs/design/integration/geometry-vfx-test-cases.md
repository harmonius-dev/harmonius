# Geometry ↔ VFX Integration Test Cases

All tests are CI-runnable: no GPU required for CPU-side contract tests. Tests that reference compute
shaders use a CPU reference implementation to validate the contract.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-7.1.1.1 | Surface query hits meshlet returns TBN | IR-7.1.1 |
| TC-IR-7.1.1.2 | Surface query miss returns `None` | IR-7.1.1 |
| TC-IR-7.1.1.3 | TBN orthogonality within tolerance | IR-7.1.1 |
| TC-IR-7.1.1.4 | UV interpolation across triangle | IR-7.1.1 |
| TC-IR-7.1.1.5 | MaterialId carried to VFX (neg if missing) | IR-7.1.1 |
| TC-IR-7.1.2.1 | SDF sample inside body returns negative | IR-7.1.2 |
| TC-IR-7.1.2.2 | SDF sample outside returns positive | IR-7.1.2 |
| TC-IR-7.1.2.3 | SDF gradient points outward | IR-7.1.2 |
| TC-IR-7.1.2.4 | SDF out-of-volume clamps to max_distance | IR-7.1.2 |
| TC-IR-7.1.3.1 | Heightfield sample matches bake | IR-7.1.3 |
| TC-IR-7.1.3.2 | OOB sample returns border (FM-2) | IR-7.1.3 |
| TC-IR-7.1.3.3 | Heightfield normal matches ddx/ddy | IR-7.1.3 |
| TC-IR-7.1.4.1 | FractureEvent triggers preset spawn | IR-7.1.4 |
| TC-IR-7.1.4.2 | Missing preset falls back to default | IR-7.1.4 |
| TC-IR-7.1.4.3 | Burst particle count equals preset | IR-7.1.4 |
| TC-IR-7.1.4.4 | Fracture below threshold no spawn (neg) | IR-7.1.4 |
| TC-IR-7.1.5.1 | Decal affects only overlapping meshlets | IR-7.1.5 |
| TC-IR-7.1.5.2 | Decal miss returns empty job list | IR-7.1.5 |
| TC-IR-7.1.6.1 | Spark alignment matches hit tangent | IR-7.1.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-7.1.1.N1 | `MaxDistance = 0` returns no hit | IR-7.1.1 |
| TC-IR-7.1.2.N1 | Missing SDF asset (FM-1) logs fallback | IR-7.1.2 |
| TC-IR-7.1.4.N1 | Unknown MaterialId uses default (FM-4) | IR-7.1.4 |
| TC-IR-7.1.5.N1 | Decal larger than mesh still bounded | IR-7.1.5 |
| TC-IR-7.1.1.N2 | Surface query reply after frame drops (FM-3) | IR-7.1.1 |
| TC-IR-7.1.1.N3 | `CH-28` full drops oldest (FM-5) | IR-7.1.1 |

### Test case details

1. **TC-IR-7.1.1.1** -- Input: a `SurfaceQueryRequest` with origin `(0,5,0)`, direction `(0,-1,0)`,
   `max_distance = 10.0`, flags `NeedNormal | NeedTangent`, against a unit cube at origin. Expected:
   a `SurfaceHit` with position `(0,0.5,0)`, normal `(0,1,0)`, tangent and bitangent mutually
   orthogonal, and entity matching the cube.
2. **TC-IR-7.1.1.2** -- Input: ray shooting into empty space. Expected: `Option<SurfaceHit>` is
   `None`; request_id still returned in reply.
3. **TC-IR-7.1.1.3** -- Input: 100 hits across varied normals. Expected:
   `dot(tangent, normal) < 1e-4`, `dot(bitangent, normal) < 1e-4`, `dot(tangent, bitangent) < 1e-4`
   for every hit.
4. **TC-IR-7.1.1.4** -- Input: ray barycentrics (0.25, 0.5, 0.25). Expected: returned UV equals
   `0.25*uv0 + 0.5*uv1 + 0.25*uv2` exactly (within fp epsilon).
5. **TC-IR-7.1.1.5** -- Input: query without `NeedMaterial` flag on a textured mesh. Expected:
   `SurfaceHit.material == MaterialId::EMPTY`; with the flag, `MaterialId` is the mesh's id.
6. **TC-IR-7.1.2.1** -- Input: sample SDF at voxel center inside a baked sphere SDF. Expected:
   negative distance, magnitude equal to distance-from-center minus radius.
7. **TC-IR-7.1.2.2** -- Input: sample SDF outside baked surface by 0.3. Expected: value
   `+0.3 ± 0.01`.
8. **TC-IR-7.1.2.3** -- Input: gradient via finite diff at outside-point. Expected: normalized
   gradient matches the analytical outward normal to within 1e-2.
9. **TC-IR-7.1.2.4** -- Input: sample way outside the SDF volume. Expected: returned value equals
   `MeshSdfAsset.max_distance`; no read outside the buffer.
10. **TC-IR-7.1.3.1** -- Input: heightfield built from flat plane at y=2.0, sample (5, 5). Expected:
    height `2.0`, normal `(0,1,0)`.
11. **TC-IR-7.1.3.2** -- Input: heightfield 64×64 at (0,0) to (32,32), sample (100, 100). Expected:
    border value (per-FM-2), e.g., nearest clamp to (31, 31); no OOB read.
12. **TC-IR-7.1.3.3** -- Input: heightfield with analytic slope; compute normal via finite diff.
    Expected: normal matches within 1e-3.
13. **TC-IR-7.1.4.1** -- Input:
    `FractureEvent { material: MaterialId::Stone, impact_energy: 50.0 }`. Expected:
    `ParticleSpawnBatch.count == preset.particle_count` with `preset` from
    `FracturePresetTable.presets[Stone]`.
14. **TC-IR-7.1.4.2** -- Input: event with a material id not in the table. Expected: spawn uses
    `FracturePresetTable.default`; overflow log emitted (FM-4).
15. **TC-IR-7.1.4.3** -- Input: preset with `particle_count = 64`. Expected: exactly 64 spawn
    batches enqueued to the VFX system.
16. **TC-IR-7.1.4.4** -- Input: `FractureEvent { impact_energy: 0.5 }` with preset threshold 5.0.
    Expected: no spawn enqueued.
17. **TC-IR-7.1.5.1** -- Input: decal OBB intersects 3 of 12 meshlets. Expected:
    `DecalProjectionJob` contains exactly those 3 meshlet ids.
18. **TC-IR-7.1.5.2** -- Input: decal OBB entirely outside all meshlet AABBs. Expected: empty job
    list returned; no compute dispatch.
19. **TC-IR-7.1.6.1** -- Input: spark emitter with `align_to_surface = true`, hit on 45° slope.
    Expected: particle orientation quaternion places `up` along hit normal.
20. **TC-IR-7.1.1.N1** -- Input: `max_distance = 0.0`. Expected: `None` hit even if ray origin is
    exactly on a surface.
21. **TC-IR-7.1.2.N1** -- Input: VFX reads SDF for a mesh with `MeshSdfAsset` entry missing.
    Expected: SDF collision disabled for the mesh; debug log emits `FM-1`.
22. **TC-IR-7.1.4.N1** -- Covered by TC-IR-7.1.4.2.
23. **TC-IR-7.1.5.N1** -- Input: decal 10x larger than the target mesh. Expected: projection job
    clamps to the meshlet AABB; no unbounded compute workload.
24. **TC-IR-7.1.1.N2** -- Input: surface query enqueued at Phase 7 boundary end. Expected: reply
    arriving next frame is discarded; `FM-3` counter increments.
25. **TC-IR-7.1.1.N3** -- Input: queue 400 surface queries against cap=256 channel. Expected: 144
    oldest requests dropped; `FM-5` counter increments by 144.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-7.1.1.B1 | 1000 surface queries against 5k meshlet scene | < 0.5 ms | IR-7.1.1 |
| TC-IR-7.1.2.B1 | 10000 SDF samples (CPU reference) | < 0.2 ms | IR-7.1.2 |
| TC-IR-7.1.3.B1 | 4096 heightfield samples | < 0.05 ms | IR-7.1.3 |
| TC-IR-7.1.4.B1 | Fracture burst spawn 1024 particles | < 0.1 ms | IR-7.1.4 |
| TC-IR-7.1.5.B1 | Decal projection 32 meshlets | < 0.1 ms | IR-7.1.5 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
