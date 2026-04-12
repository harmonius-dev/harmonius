# Physics ↔ World Geometry Integration Test Cases

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.8.1.1 | Tri-mesh collider from asset | IR-3.8.1 |
| TC-IR-3.8.1.2 | Simplified collision mesh from visual | IR-3.8.1 |
| TC-IR-3.8.1.3 | Dynamic body rests on tri-mesh | IR-3.8.1 |
| TC-IR-3.8.2.1 | Heightfield collider from tile | IR-3.8.2 |
| TC-IR-3.8.2.2 | Character walks on heightfield | IR-3.8.2 |
| TC-IR-3.8.2.3 | u16 dequantization precision | IR-3.8.2 |
| TC-IR-3.8.3.1 | Collision ignores visual LOD | IR-3.8.3 |
| TC-IR-3.8.3.2 | Physics res constant vs camera | IR-3.8.3 |
| TC-IR-3.8.4.1 | Hole allows fall-through | IR-3.8.4 |
| TC-IR-3.8.4.2 | Hole blocks adjacent cell | IR-3.8.4 |
| TC-IR-3.8.4.3 | Hole mask layout match | IR-3.8.4 |
| TC-IR-3.8.5.1 | Voxel chunk generates collider | IR-3.8.5 |
| TC-IR-3.8.5.2 | Voxel edit rebuilds collision | IR-3.8.5 |
| TC-IR-3.8.5.3 | Entity collides with voxel mesh | IR-3.8.5 |
| TC-IR-3.8.5.4 | Index triplet layout verified | IR-3.8.5 |
| TC-IR-3.8.6.1 | Layer filter blocks contact | IR-3.8.6 |
| TC-IR-3.8.6.2 | Layer filter allows contact | IR-3.8.6 |

Input and expected output per test:

1. **TC-IR-3.8.1.1** -- Input: cube mesh asset. Expected: `ColliderShape::TriangleMesh` with 12
   triangles.
2. **TC-IR-3.8.1.2** -- Input: 10K-tri visual mesh run through the bake pipeline. Expected:
   collision mesh < 1K triangles after quadric-error decimation.
3. **TC-IR-3.8.1.3** -- Input: sphere dropped onto a mesh floor. Expected: sphere stops; contacts
   are generated against the triangle mesh.
4. **TC-IR-3.8.2.1** -- Input: 257x257 tile with a central peak. Expected: collider height samples
   match the tile height at the peak within quantization tolerance.
5. **TC-IR-3.8.2.2** -- Input: character controller placed on a terrain tile. Expected: grounded
   state detected and smooth walking without pops.
6. **TC-IR-3.8.2.3** -- Input: `min=0`, `max=1000`, `sample=32768`. Expected: `sample_height`
   returns `500.0 +/- 0.02`. The tolerance of 0.02 reflects the ~1.5 cm precision of 16-bit
   quantization across a 1 km height range.
7. **TC-IR-3.8.3.1** -- Input: entity at visual LOD 3. Expected: collision query uses the
   full-resolution collision shape regardless of visual LOD.
8. **TC-IR-3.8.3.2** -- Input: camera far from terrain. Expected: heightfield collision shape is
   unchanged by camera distance.
9. **TC-IR-3.8.4.1** -- Input: tile with a hole at cell `(5, 5)`. Expected: entity dropped above the
   hole cell falls through with zero collision response.
10. **TC-IR-3.8.4.2** -- Input: hole at `(5, 5)`, entity at `(6, 5)`. Expected: normal collision
    response against the neighbouring cell.
11. **TC-IR-3.8.4.3** -- Input: a `TerrainHole { mask, resolution }`. Expected:
    `HeightfieldCollider.hole_mask` uses the same layout, so bit `(r * resolution + c)` selects the
    same cell in both domains without conversion.
12. **TC-IR-3.8.5.1** -- Input: a 16^3 voxel chunk, half solid. Expected: triangle mesh produced by
    Marching Cubes.
13. **TC-IR-3.8.5.2** -- Input: remove voxel at `(4, 4, 4)`. Expected: updated collision mesh is
    visible on the next frame.
14. **TC-IR-3.8.5.3** -- Input: sphere rolling over a voxel-derived terrain surface. Expected:
    contacts are generated at the surface.
15. **TC-IR-3.8.5.4** -- Input: meshed 16^3 chunk. Expected: `VoxelCollisionMesh.indices` is
    `Vec<[u32; 3]>`, directly usable as `TriMeshData`.
16. **TC-IR-3.8.6.1** -- Input: projectile layer `0x04`, terrain filter `0x01`. Expected: no
    collision contact generated.
17. **TC-IR-3.8.6.2** -- Input: character layer `0x01`, terrain filter `0x01`. Expected: normal
    collision response.

## Negative / Error-Path Tests

| ID | Test | FM |
|----|------|-----|
| TC-IR-3.8.E1 | Over-budget tri count | FM-1 |
| TC-IR-3.8.E2 | Heightfield not yet loaded | FM-2 |
| TC-IR-3.8.E3 | Voxel remesh over budget | FM-3 |
| TC-IR-3.8.E4 | Hole mask resolution mismatch | FM-4 |
| TC-IR-3.8.E5 | Scale mismatch rejected | FM-5 |
| TC-IR-3.8.E6 | Remesh queue overflow | FM-6 |
| TC-IR-3.8.E7 | Unresolved material handle | FM-7 |

Input and expected output per test:

1. **TC-IR-3.8.E1** -- Input: 300K-triangle bake targeted at the mobile platform. Expected: fallback
   to the pre-baked lower-LOD collision mesh and a single `warn` log at bake time.
2. **TC-IR-3.8.E2** -- Input: character on a tile whose heightfield is still loading. Expected:
   temporary infinite-plane collider inserted; tile load is promoted to a blocking fence; `debug`
   log emitted.
3. **TC-IR-3.8.E3** -- Input: 64 voxel chunks edited in a single frame. Expected: remesh budget
   exceeded, deferred chunks retain previous collision mesh, `debug` log with deferred count.
4. **TC-IR-3.8.E4** -- Input: hole mask resolution does not match tile resolution. Expected: mask
   re-baked at tile resolution using nearest-neighbour sampling; single `warn` log.
5. **TC-IR-3.8.E5** -- Input: heightfield with `scale.x * samples_x != tile.world_size`. Expected:
   collider rejected by the build step with an `error` log.
6. **TC-IR-3.8.E6** -- Input: 500 voxel edits queued faster than physics can drain them. Expected:
   producer coalesces edits per chunk, drops older events, emits a `debug` log with drop count.
7. **TC-IR-3.8.E7** -- Input: `PhysicsMaterialHandle` pointing to an unloaded asset. Expected:
   default material (friction 0.5, restitution 0.0) used; single `warn` per handle.

All negative tests run under `cargo test` in CI with no external fixtures. Log output is asserted
via the in-process log capture used by the rest of the physics test suite.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.8.1.B1 | Tri-mesh narrowphase 10K pairs | < 2 ms (R-4.2.NF2) | IR-3.8.1 |
| TC-IR-3.8.2.B1 | Heightfield build 257x257 | < 5 ms CPU | IR-3.8.2 |
| TC-IR-3.8.2.B2 | 200 characters on terrain | < 4 ms total physics | IR-3.8.2 |
| TC-IR-3.8.5.B1 | Voxel remesh 16^3 chunk | < 5 ms CPU | IR-3.8.5 |
| TC-IR-3.8.5.B2 | Collider insert 100 colliders | < 0.5 ms | IR-3.8.5 |

Benchmarks run under `cargo bench` in CI and are tracked against the Performance Budget table in the
companion design document.

The `TC-IR-3.8.2.B2` 200-character-on-terrain benchmark directly exercises the character-count
performance budget from US-9.4.10; a regression there also counts as a US-9.4.10 regression.
