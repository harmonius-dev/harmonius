# Rendering ↔ UI Framework Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.6.1.1 | UI pass in render graph | 10 widgets on screen | UI pass executes after tonemap | IR-3.6.1 |
| TC-IR-3.6.1.2 | UI pass reads scene color | Scene with sky, UI overlay | UI composited on top of scene | IR-3.6.1 |
| TC-IR-3.6.2.1 | QuadBatcher batches widgets | 100 same-atlas widgets | Single indirect draw call | IR-3.6.2 |
| TC-IR-3.6.2.2 | QuadBatcher splits on atlas | 50 atlas-A + 50 atlas-B | Two draw calls | IR-3.6.2 |
| TC-IR-3.6.3.1 | MSDF text renders clearly | "Hello" at 24px | Crisp anti-aliased glyphs | IR-3.6.3 |
| TC-IR-3.6.3.2 | MSDF text scales without blur | Same text at 12px and 48px | Sharp at both sizes | IR-3.6.3 |
| TC-IR-3.6.3.3 | 5000 glyphs in one frame | Full chat log + HUD text | All glyphs render < 2 ms | IR-3.6.3 |
| TC-IR-3.6.4.1 | World-space panel depth tests | Panel behind a wall | Panel occluded by wall | IR-3.6.4 |
| TC-IR-3.6.4.2 | World-space panel receives light | Panel in lit area | Panel shaded by scene lights | IR-3.6.4 |
| TC-IR-3.6.5.1 | RTT preview renders model | 3D sword model in UI slot | Correct model in UI quad | IR-3.6.5 |
| TC-IR-3.6.5.2 | RTT updates on change | Swap model in preview | New model appears next frame | IR-3.6.5 |
| TC-IR-3.6.6.1 | UI after tonemap before grain | Enable film grain + UI | UI unaffected by grain | IR-3.6.6 |
| TC-IR-3.6.7.1 | Nameplate tracks 3D position | Entity at world (0,10,0) | Nameplate at projected screen XY | IR-3.6.7 |
| TC-IR-3.6.7.2 | Nameplate culled behind camera | Entity behind camera | Nameplate not rendered | IR-3.6.7 |
| TC-IR-3.6.7.3 | 200 nameplates at 60 fps | 200 entities with nameplates | All visible, frame time < 16 ms | IR-3.6.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.6.1.B1 | Full HUD render | < 2 ms GPU, < 50 draws | IR-3.6.1 |
| TC-IR-3.6.2.B1 | QuadBatcher 500 widgets | < 0.5 ms CPU | IR-3.6.2 |
| TC-IR-3.6.3.B1 | MSDF 5000 glyphs | < 1 ms GPU | IR-3.6.3 |
| TC-IR-3.6.5.B1 | RTT 256x256 preview | < 0.5 ms GPU | IR-3.6.5 |
| TC-IR-3.6.7.B1 | 200 nameplate projections | < 0.1 ms CPU | IR-3.6.7 |
