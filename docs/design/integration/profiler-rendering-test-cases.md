# Profiler ↔ Rendering Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.7.1.1 | Shadow pass timed | Scene with 1 directional light | GpuPassTiming for "ShadowPass" | IR-5.7.1 |
| TC-IR-5.7.1.2 | All passes timed | Full frame with 8 passes | 8 GpuPassTiming entries | IR-5.7.1 |
| TC-IR-5.7.1.3 | Pass durations non-negative | 100 frames | All duration_ms >= 0.0 | IR-5.7.1 |
| TC-IR-5.7.2.1 | Draw call count accurate | 50 opaque + 10 transparent meshes | draw_calls == 60 (or fewer with batching) | IR-5.7.2 |
| TC-IR-5.7.2.2 | Triangle count accurate | Mesh with 1000 known triangles | triangles == 1000 | IR-5.7.2 |
| TC-IR-5.7.2.3 | Meshlet cull ratio | 1000 meshlets, camera facing 50% | meshlets_culled approximately 500 | IR-5.7.2 |
| TC-IR-5.7.3.1 | VRAM texture breakdown | Load 10 textures (100 MB total) | vram_textures >= 100 MB | IR-5.7.3 |
| TC-IR-5.7.3.2 | VRAM buffer breakdown | Upload 50 MB of mesh buffers | vram_buffers >= 50 MB | IR-5.7.3 |
| TC-IR-5.7.3.3 | VRAM render target tracking | 4 render targets allocated | vram_render_targets > 0 | IR-5.7.3 |
| TC-IR-5.7.4.1 | GPU timeline aligns with CPU | 1 frame captured | GPU pass begins after CPU submit | IR-5.7.4 |
| TC-IR-5.7.4.2 | 2-frame readback latency | Capture frame N, read at N+2 | Frame N data available at N+2 | IR-5.7.4 |
| TC-IR-5.7.5.1 | GPU timeline shows pass names | 5 named render passes | 5 labeled bars in timeline view | IR-5.7.5 |
| TC-IR-5.7.5.2 | Pass timing sum matches total | 8 pass timings | Sum within 10% of total GPU time | IR-5.7.5 |
| TC-IR-5.7.6.1 | Per-view draw list stats | 2 cameras (main + shadow) | Separate draw counts per view | IR-5.7.6 |
| TC-IR-5.7.7.1 | Queries absent in shipping | Compile with shipping profile | Zero query pool allocations | IR-5.7.7 |
| TC-IR-5.7.7.2 | GpuScope is no-op in shipping | Compile with shipping profile | GpuScope::begin compiles to nothing | IR-5.7.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.7.1.B1 | GpuScope begin+end cost | < 1 us GPU | IR-5.7.1 |
| TC-IR-5.7.4.B1 | Query readback latency | <= 2 frames | IR-5.7.4 |
| TC-IR-5.7.7.B1 | Shipping build GPU overhead | 0 ms (compiled out) | IR-5.7.7 |
| TC-IR-5.7.2.B1 | Stat collection per frame | < 0.05 ms CPU | IR-5.7.2 |
