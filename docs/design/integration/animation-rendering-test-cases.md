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

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.4.1.B1 | 1000 skinning dispatches | < 2 ms GPU | IR-1.4.1 |
| TC-IR-1.4.2.B1 | 16 morph targets 500 ents | < 0.5 ms GPU | IR-1.4.2 |
| TC-IR-1.4.5.B1 | 1000 instanced crowd | < 1 ms GPU | IR-1.4.5 |
