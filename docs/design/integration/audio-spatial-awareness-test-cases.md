# Audio ↔ Spatial Awareness Integration Test Cases

All tests are CI-runnable (`cargo test -p harmonius-integration-audio-sa`). No mocks; real BVH, real
`AcousticMaterialTable`, real `crossbeam_channel`. The audio thread is exercised by a fake driver
that pumps `try_recv` on the test thread (same interface as the real driver).

## Scope

3D only. 2D/2.5D spatial audio propagation is intentionally out of scope per the companion design
[audio-spatial-awareness.md](audio-spatial-awareness.md).

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.9.1.1 | LOS unoccluded | No wall between src/lis | occlusion == 1.0 | IR-1.9.1 |
| TC-IR-1.9.1.2 | Wall fully occludes | Thick wall blocks all | occlusion near 0.0 | IR-1.9.1 |
| TC-IR-1.9.1.3 | Partial occlusion | Half rays blocked | occlusion near 0.5 | IR-1.9.1 |
| TC-IR-1.9.2.1 | Stone low absorption | Rays hit stone | band_loss low values | IR-1.9.2 |
| TC-IR-1.9.2.2 | Carpet high absorption | Rays hit carpet | band_loss[high] large | IR-1.9.2 |
| TC-IR-1.9.2.3 | Glass low transmission | Rays through glass | Moderate attenuation | IR-1.9.2 |
| TC-IR-1.9.3.1 | Result feeds audio | PropagationResult set | Voice filter applied | IR-1.9.3 |
| TC-IR-1.9.3.2 | Reflections produce taps | 2 wall bounces | 2 ReflectionTap entries | IR-1.9.3 |
| TC-IR-1.9.3.3 | Reverb send from geom | Enclosed room | reverb_send near 1.0 | IR-1.9.3 |
| TC-IR-1.9.4.1 | 8 rays 4 blocked | occlusion_rays=8 | occlusion == 0.5 | IR-1.9.4 |
| TC-IR-1.9.4.2 | 0 rays skips | occlusion_rays=0 | No trace, default 1.0 | IR-1.9.4 |
| TC-IR-1.9.5.1 | Static source cached | Source not moved | No re-trace | IR-1.9.5 |
| TC-IR-1.9.5.2 | Moved source re-traces | Source moved 1m | New trace dispatched | IR-1.9.5 |
| TC-IR-1.9.5.3 | Amortized rotation | 100 sources, N=4 | 25 traced per frame | IR-1.9.5 |
| TC-IR-1.9.5.4 | New source pre-trace | Spawn + 0 frames | LOS default applied | IR-1.9.5 |
| TC-IR-1.9.3.4 | MPSC drain updates snap | Send 3, drain once | Snapshot has all 3 | IR-1.9.3 |
| TC-IR-1.9.3.5 | Worker write disjoint | par_for_each 100 src | No race, all slots | IR-1.9.3 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.9.1.N1 | BVH not ready | Index unbuilt | LOS default 1.0 applied | IR-1.9.1 |
| TC-IR-1.9.2.N1 | Material missing | Hit ent no AcMat | Stone default used | IR-1.9.2 |
| TC-IR-1.9.3.N1 | MPSC full drops | Fill 256 then +1 | try_send err, prev kept | IR-1.9.3 |
| TC-IR-1.9.3.N2 | Despawned orphan | Send then despawn | Drain discards silently | IR-1.9.3 |
| TC-IR-1.9.3.N3 | Empty drain | No pending results | Snapshot unchanged | IR-1.9.3 |
| TC-IR-1.9.4.N1 | All rays blocked | 8/8 blocked | occlusion=0, max LP | IR-1.9.4 |
| TC-IR-1.9.5.N1 | Despawned mid-trace | Despawn in Phase 3 | No panic, dropped | IR-1.9.5 |
| TC-IR-1.9.5.N2 | Reflection overflow | 16 reflections | 8 kept by gain | IR-1.9.3 |

1. **TC-IR-1.9.1.N1** -- run system before BVH build completes; assert all sources receive
   `PropagationResult { occlusion: 1.0, band_loss: [0.0; 3], reverb_send: 0.0 }`.
2. **TC-IR-1.9.2.N1** -- hit entity lacks `AcousticMaterial`; assert resolved material equals the
   default stone constant.
3. **TC-IR-1.9.3.N1** -- pre-fill channel to capacity 256, then issue one more `try_send`; assert
   `Err(TrySendError::Full)` and that audio thread still reads the prior snapshot entry.
4. **TC-IR-1.9.3.N2** -- despawn source after send but before drain; assert drain does not insert
   into `PropagationSnapshot` and does not panic.
5. **TC-IR-1.9.3.N3** -- drain with empty channel; assert snapshot contents byte-equal to prior.
6. **TC-IR-1.9.4.N1** -- geometry surrounds source; assert `occlusion == 0.0` and LP filter cutoff
   equals configured minimum.
7. **TC-IR-1.9.5.N1** -- despawn entity mid-Phase 3; assert no panic and the stale slot is not sent.
8. **TC-IR-1.9.5.N2** -- synthesize 16 candidate reflections; assert `reflection_count == 8` and the
   retained taps are the top 8 by gain.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.9.1.B1 | 100 sources x 100 rays | < 2 ms | IR-1.9.1 |
| TC-IR-1.9.5.B1 | 25 sources traced/frame | < 0.5 ms | IR-1.9.5 |
| TC-IR-1.9.3.B1 | MPSC drain 256 results | < 0.05 ms | IR-1.9.3 |
| TC-IR-1.9.3.B2 | MPSC `try_send` hot path | < 0.001 ms/call | IR-1.9.3 |
| TC-IR-1.9.3.B3 | par_for_each store write | < 0.2 ms / 100 src | IR-1.9.3 |
