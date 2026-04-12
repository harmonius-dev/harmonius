# Animation ↔ VFX Integration Test Cases

All tests are CI-runnable via `cargo test` or `cargo bench`; none require GPU hardware (particle
simulation is stubbed via a deterministic CPU fake when running tests).

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.6.1.1 | VfxSpawn fires | VfxSpawn at t=1 | Emitter spawned | IR-1.6.1 |
| TC-IR-1.6.1.2 | Spawn at bone pos | Event on hand bone | Emitter at hand pos | IR-1.6.1 |
| TC-IR-1.6.1.3 | No dup same frame | 1 event, 1 frame | Exactly 1 emitter | IR-1.6.1 |
| TC-IR-1.6.1.4 | Same-frame visibility | VfxSpawn at t=1 | Emitter visible on frame 1 | IR-1.6.1 |
| TC-IR-1.6.2.1 | Trail follows bone | Swing arm anim | Ribbon on arc | IR-1.6.2 |
| TC-IR-1.6.2.2 | Trail stops idle | Arm stops moving | No new ribbon pts | IR-1.6.2 |
| TC-IR-1.6.3.1 | Weapon trail on | active=true | Spawn rate > 0 | IR-1.6.3 |
| TC-IR-1.6.3.2 | Weapon trail off | active=false | Spawn rate == 0 | IR-1.6.3 |
| TC-IR-1.6.3.3 | Trail matches hit | HitWindow timing | Active in window only | IR-1.6.3 |
| TC-IR-1.6.4.1 | LOD Full keeps VFX | Distance < 10 m | rate_scale == 1.0 | IR-1.6.4 |
| TC-IR-1.6.4.2 | LOD Reduced | Distance 10-30 m | rate_scale == 0.75 | IR-1.6.4 |
| TC-IR-1.6.4.3 | LOD HalfRate | Distance 30-100 m | rate_scale == 0.5 | IR-1.6.4 |
| TC-IR-1.6.4.4 | LOD Vat culls VFX | Distance > 100 m | rate_scale == 0, inactive | IR-1.6.4 |
| TC-IR-1.6.5.1 | Bone attach syncs | Parent anim plays | Child follows bone | IR-1.6.5 |
| TC-IR-1.6.5.2 | Parent despawn | Parent removed | Child cleaned up | IR-1.6.5 |

## Negative / Failure-Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.6.1.N1 | Missing VfxAsset | Invalid handle | No spawn, warn logged | IR-1.6.1 |
| TC-IR-1.6.1.N2 | Unknown payload variant | Non-VFX event | Observer skips, no spawn | IR-1.6.1 |
| TC-IR-1.6.2.N1 | Trail without bone | Detached ribbon | Fallback to entity xf | IR-1.6.2 |
| TC-IR-1.6.3.N1 | Trail entity missing | Toggle on stale id | Warn logged, no panic | IR-1.6.3 |
| TC-IR-1.6.4.N1 | Unknown LOD tier | Out-of-range tier | Clamp to `Full`, warn | IR-1.6.4 |
| TC-IR-1.6.5.N1 | Invalid bone_index | bone_index too high | Clamp to root bone, warn | IR-1.6.5 |
| TC-IR-1.6.5.N2 | Parent already dead | Parent entity stale | Despawn child, no panic | IR-1.6.5 |

## Benchmarks

All benchmarks run under `cargo bench --bench animation_vfx` on CI reference hardware.

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.6.1.B1 | 100 VFX spawns/frame | < 0.3 ms | IR-1.6.1 |
| TC-IR-1.6.2.B1 | 200 trail emitters sync | < 0.2 ms | IR-1.6.2 |
| TC-IR-1.6.3.B1 | 1000 WeaponTrail toggles/frame | < 0.1 ms | IR-1.6.3 |
| TC-IR-1.6.4.B1 | 500 emitters LOD remap | < 0.15 ms | IR-1.6.4 |
| TC-IR-1.6.5.B1 | 500 bone attachments | < 0.3 ms | IR-1.6.5 |

## Notes

1. **TC-IR-1.6.1.4** enforces the "no one-frame delay" rule: the emitter must be queryable in the
   same frame the animation event sampled. Test asserts `World::query::<&ParticleEmitter>` returns
   the new entity before Phase 8 particle sim runs.
2. **TC-IR-1.6.4.*** uses the exact `LodTier` -> `rate_scale` mapping from the design's LOD table
   (`Full=1.0`, `Reduced=0.75`, `HalfRate=0.5`, `Vat=0.0`).
3. **Negative tests** fail the test body if the expected warning is not emitted (captured via a test
   log sink) or if a panic occurs.
4. **2D/2.5D out of scope** -- no 2D sprite-frame VFX tests; covered by the 2D sprite/VFX
   integration test case file.
