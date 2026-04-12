# Animation ↔ Rendering Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.4.1.1 | Skinning dispatch | 1 entity + blend | Compute dispatched | IR-1.4.1 |
| TC-IR-1.4.1.2 | LBS vs DQS select | Mode::Dqs | DQS pipeline used | IR-1.4.1 |
| TC-IR-1.4.2.1 | Morph before skin | 4 morph targets | Deltas pre-skin | IR-1.4.2 |
| TC-IR-1.4.2.2 | Zero morph skip | All weights 0.0 | Morph pass skipped | IR-1.4.2 |
| TC-IR-1.4.3.1 | LOD Full evals all | Distance < 10 m | All bones evaluated | IR-1.4.3 |
| TC-IR-1.4.3.2 | LOD VAT skips skin | Distance > 100 m | VAT playback only | IR-1.4.3 |
| TC-IR-1.4.3.3 | LOD HalfRate | Distance 30-60 m | Eval every 2nd frame | IR-1.4.3 |
| TC-IR-1.4.4.1 | Snapshot has bones | Phase 7 complete | BonePalette in frame | IR-1.4.4 |
| TC-IR-1.4.4.2 | Render reads snap | Render thread tick | No ECS access | IR-1.4.4 |
| TC-IR-1.4.5.1 | 1000 inst 1 batch | Same skel + clip | Single arena dispatch | IR-1.4.5 |
| TC-IR-1.4.5.2 | Mixed skel batch | 3 skeleton types | 3 dispatches | IR-1.4.5 |

## Negative / Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.4.1.N1 | Arena full demotion | Arena at capacity | Excess -> ReducedBones | IR-1.4.1 |
| TC-IR-1.4.1.N2 | Arena full to VAT | Arena + no bones left | Excess -> Vat fallback | IR-1.4.1 |
| TC-IR-1.4.1.N3 | GPU timeout batch | Dispatch > budget | Halve batch, retry | IR-1.4.1 |
| TC-IR-1.4.1.N4 | Zero blend weight | All weights 0.0 | Bind-pose buffer used | IR-1.4.1 |
| TC-IR-1.4.2.N1 | Morph target overflow | 32 active targets | Clamp to first 16 | IR-1.4.2 |
| TC-IR-1.4.2.N2 | No morph buffer | `morph_buffer = None` | Skin-only dispatch | IR-1.4.2 |
| TC-IR-1.4.3.N1 | Invalid LOD tier | Corrupted tier value | Fallback Full tier | IR-1.4.3 |
| TC-IR-1.4.3.N2 | HalfRate stale > 2 | 3 missed frames | Force Full eval next | IR-1.4.3 |
| TC-IR-1.4.4.N1 | Handle generation stale | Freed buffer handle | Error, skip draw | IR-1.4.4 |
| TC-IR-1.4.5.N1 | HashMap guard | Instrument hot path | Zero HashMap ops | IR-1.4.5 |

1. **TC-IR-1.4.1.N1** -- verifies `GpuArenaBuffer` overflow path demotes excess entities to the
   `ReducedBones` LOD tier rather than allocating new GPU memory.
2. **TC-IR-1.4.1.N2** -- verifies secondary fallback to `Vat` when `ReducedBones` budget is also
   exhausted.
3. **TC-IR-1.4.1.N3** -- verifies GPU timeout recovery halves the instance batch and resubmits on
   the next frame.
4. **TC-IR-1.4.1.N4** -- verifies zero blend weight path binds the bind-pose vertex buffer directly,
   skipping the skinning dispatch.
5. **TC-IR-1.4.2.N1** -- verifies `MorphTargets` overflow is clamped to the first 16 targets without
   a runtime panic.
6. **TC-IR-1.4.2.N2** -- verifies the optional `morph_buffer` field, when `None`, issues a
   skinning-only dispatch with no morph pass.
7. **TC-IR-1.4.3.N1** -- verifies an invalid or corrupted `AnimationLodTier` value falls back to
   `Full` tier and logs a warning.
8. **TC-IR-1.4.3.N2** -- verifies `HalfRate` entities with more than two missed updates force a full
   evaluation on the next frame to avoid pose staleness.
9. **TC-IR-1.4.4.N1** -- verifies a `Handle<GpuBuffer>` with a stale generation counter returns an
   error from the resource manager and the skinned draw call is skipped.
10. **TC-IR-1.4.5.N1** -- static/runtime assertion that the instanced skinning grouping hot path
    makes zero `HashMap` allocations or lookups. Uses a runtime-toggleable debug counter.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.4.1.B1 | 1000 skinning dispatches | < 2 ms GPU | IR-1.4.1 |
| TC-IR-1.4.2.B1 | 16 morph targets 500 ents | < 0.5 ms GPU | IR-1.4.2 |
| TC-IR-1.4.3.B1 | LOD tier transition cost | < 0.1 ms CPU | IR-1.4.3 |
| TC-IR-1.4.3.B2 | HalfRate skip overhead | < 0.05 ms CPU | IR-1.4.3 |
| TC-IR-1.4.5.B1 | 1000 instanced crowd | < 1 ms GPU | IR-1.4.5 |
| TC-IR-1.4.5.B2 | Grouping sort 10k inst | < 0.2 ms CPU | IR-1.4.5 |

1. **TC-IR-1.4.3.B1** -- measures the per-frame cost of switching 500 entities between LOD tiers
   (`Full` <-> `ReducedBones` <-> `HalfRate` <-> `Vat`) as entities cross distance thresholds.
2. **TC-IR-1.4.3.B2** -- measures the CPU overhead of skipping the skinning dispatch for `HalfRate`
   entities on their off-frames, binding the prior vertex buffer directly.
3. **TC-IR-1.4.5.B2** -- measures the cost of sorting the instanced skinning grouping `Vec` for
   10,000 skinned entities on the Phase 6 hot path. Ensures no `HashMap` regression.

All benchmarks run under `cargo bench` and are CI-executable. Targets are measured on the reference
hardware profile defined in `docs/design/core-runtime/game-loop.md`.
