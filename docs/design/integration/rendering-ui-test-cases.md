# Rendering ↔ UI Framework Integration Test Cases

All tests are CI-runnable against the real `RenderGraph`, `QuadBatcher`, `SdfAtlas`, and
`NameplateSystem` implementations. No mocks are used — fakes only where an interface has no real
implementation yet (e.g. a headless `GpuDevice` fake that implements the same trait as the Metal /
Vulkan / D3D12 backends).

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.6.1.1 | UI pass in render graph | 10 widgets | UI runs in `RenderPhase::UI` | IR-3.6.1 |
| TC-IR-3.6.1.2 | UI reads scene color | Sky + UI overlay | UI composited on top | IR-3.6.1 |
| TC-IR-3.6.2.1 | Batch same atlas | 100 widgets, 1 atlas | 1 indirect draw | IR-3.6.2 |
| TC-IR-3.6.2.2 | Split on atlas | 50 + 50 on 2 atlases | 2 draws | IR-3.6.2 |
| TC-IR-3.6.2.3 | Split on blend state | 50 opaque + 50 additive | 2 draws | IR-3.6.2 |
| TC-IR-3.6.3.1 | MSDF crisp at 24px | "Hello" at 24px | SSIM > 0.98 vs golden | IR-3.6.3 |
| TC-IR-3.6.3.2 | MSDF scales | 12px and 48px | Sharp at both sizes | IR-3.6.3 |
| TC-IR-3.6.3.3 | 5000 glyphs | Chat + HUD | All glyphs < 2 ms GPU | IR-3.6.3 |
| TC-IR-3.6.3.4 | MSDF fallback to bitmap | Fail MSDF gen | Bitmap glyph emitted | IR-3.6.3 |
| TC-IR-3.6.3.5 | Bitmap fallback to tofu | Fail bitmap raster | `U+FFFD` glyph emitted | IR-3.6.3 |
| TC-IR-3.6.4.1 | World-space depth test | Panel behind wall | Panel occluded | IR-3.6.4 |
| TC-IR-3.6.4.2 | World-space lighting | Panel in lit area | Shaded by scene lights | IR-3.6.4 |
| TC-IR-3.6.4.3 | World-space 2D camera | Panel via Transform2D | Correct 2D projection | IR-3.6.4 |
| TC-IR-3.6.5.1 | RTT renders model | 3D sword in UI slot | Correct model in quad | IR-3.6.5 |
| TC-IR-3.6.5.2 | RTT updates on change | Swap model | New model next frame | IR-3.6.5 |
| TC-IR-3.6.5.3 | RTT persistent reused | 10 frames, persistent=true | 1 GPU alloc | IR-3.6.5 |
| TC-IR-3.6.5.4 | RTT transient aliased | persistent=false | Graph aliases target | IR-3.6.5 |
| TC-IR-3.6.6.1 | UI after tonemap | Film grain on + UI | UI unaffected by grain | IR-3.6.6 |
| TC-IR-3.6.6.2 | UI before vignette | Vignette on + UI | UI not darkened at edges | IR-3.6.6 |
| TC-IR-3.6.7.1 | Nameplate 3D project | Entity at world (0,10,0) | Correct screen XY | IR-3.6.7 |
| TC-IR-3.6.7.2 | Nameplate cull behind cam | Entity behind camera | Not rendered | IR-3.6.7 |
| TC-IR-3.6.7.3 | 200 nameplates 60 fps | 200 entities | Frame < 16 ms | IR-3.6.7 |
| TC-IR-3.6.7.4 | Nameplate 2D source | Entity with Transform2D | Uses 2D camera | IR-3.6.7 |

## Negative Tests

Negative tests verify the system gracefully handles out-of-budget, missing, or corrupted inputs. All
must pass in CI without panicking or deadlocking.

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.6.1.N1 | UI pass with 0 widgets | Empty world | Pass skipped, no crash | IR-3.6.1 |
| TC-IR-3.6.2.N1 | Batch overflow split | 10000 widgets | Split into sub-batches | IR-3.6.2 |
| TC-IR-3.6.2.N2 | Draws > mobile budget | 40 draws on mobile | Diagnostic emitted | IR-3.6.2 |
| TC-IR-3.6.3.N1 | Atlas full eviction | Fill atlas + 1 glyph | LRU evict, new glyph in | IR-3.6.3 |
| TC-IR-3.6.3.N2 | Unsupported font | Corrupt font blob | Tofu `U+FFFD` emitted | IR-3.6.3 |
| TC-IR-3.6.4.N1 | Panel off-camera | Panel behind camera | Culled, no draw | IR-3.6.4 |
| TC-IR-3.6.5.N1 | RTT persistent fail | Alloc returns OOM | Retry transient, warn | IR-3.6.5 |
| TC-IR-3.6.5.N2 | RTT transient fail | Both alloc paths OOM | Solid `clear_color` quad | IR-3.6.5 |
| TC-IR-3.6.6.N1 | Missing tonemap pass | No tonemap registered | UI renders in linear | IR-3.6.6 |
| TC-IR-3.6.7.N1 | Nameplate at w=0 | Behind camera plane | `visible=false` | IR-3.6.7 |
| TC-IR-3.6.7.N2 | Over NAMEPLATE_CAPACITY | 300 on desktop (cap 256) | Excess dropped | IR-3.6.7 |
| TC-IR-3.6.MP.N1 | MPSC channel saturated | Render thread stalled | Back-pressure | IR-3.6.1 |

## Benchmarks

Benchmarks run in CI on a headless test rig. Each benchmark asserts against a per-platform target.
Targets are enforced by the Criterion harness and fail the build on regression > 10 %.

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.6.1.B1 | Full HUD render desktop | < 2 ms GPU, <= 50 draws | IR-3.6.1 |
| TC-IR-3.6.1.B2 | Full HUD render mobile | < 3 ms GPU, <= 30 draws | IR-3.6.1 |
| TC-IR-3.6.2.B1 | QuadBatcher 500 widgets | < 0.5 ms CPU | IR-3.6.2 |
| TC-IR-3.6.2.B2 | QuadBatcher 2000 widgets | < 1.5 ms CPU | IR-3.6.2 |
| TC-IR-3.6.3.B1 | MSDF 5000 glyphs | < 1 ms GPU | IR-3.6.3 |
| TC-IR-3.6.4.B1 | 50 world-space panels | < 1 ms GPU, depth tested | IR-3.6.4 |
| TC-IR-3.6.4.B2 | Panels in 2D camera | < 0.8 ms GPU | IR-3.6.4 |
| TC-IR-3.6.5.B1 | RTT 256x256 preview | < 0.5 ms GPU | IR-3.6.5 |
| TC-IR-3.6.5.B2 | 8 RTT previews | < 2 ms GPU | IR-3.6.5 |
| TC-IR-3.6.6.B1 | UI + full post-process | Order stable, < 3 ms GPU | IR-3.6.6 |
| TC-IR-3.6.6.B2 | UI pass ordering desktop | <= 50 draws asserted | IR-3.6.6 |
| TC-IR-3.6.6.B3 | UI pass ordering mobile | <= 30 draws asserted | IR-3.6.6 |
| TC-IR-3.6.7.B1 | 200 nameplate projections | < 0.1 ms CPU | IR-3.6.7 |
| TC-IR-3.6.7.B2 | Arena nameplate allocs | 0 heap allocs / frame | IR-3.6.7 |

## Debug Tooling Tests

These tests exercise the runtime-toggleable debug overlays required by the project-wide debug
tooling rule. Every overlay must be toggleable at runtime (no recompile).

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.6.D1 | Toggle draw overlay | `ui.debug.draws = true` | Overlay same frame | IR-3.6.1 |
| TC-IR-3.6.D2 | Toggle glyph atlas | `ui.debug.atlas = true` | Atlas page as quad | IR-3.6.3 |
| TC-IR-3.6.D3 | Toggle nameplate depth | `ui.debug.nameplates = 1` | Depths overlaid | IR-3.6.7 |
| TC-IR-3.6.D4 | Toggle RTT wireframe | `ui.debug.rtt = true` | Wireframe on preview | IR-3.6.5 |
