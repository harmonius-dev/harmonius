# Rendering ↔ World Geometry Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.2.1.1 | Meshlet clusters upload to GPU | 1000 meshlet DAG | GPU buffer contains all clusters | IR-3.2.1 |
| TC-IR-3.2.1.2 | Frustum cull rejects off-screen | 100 clusters, 50 outside frustum | 50 clusters in draw list | IR-3.2.1 |
| TC-IR-3.2.2.1 | LOD selects coarsest valid cut | Object at 100m, 1-pixel threshold | Coarsest DAG level selected | IR-3.2.2 |
| TC-IR-3.2.2.2 | LOD transitions smoothly | Object moves 50m to 200m over 60 frames | No visible pop, hysteresis active | IR-3.2.2 |
| TC-IR-3.2.3.1 | V-buffer writes correct IDs | 10 instances, 100 triangles each | Each pixel has valid triangle+instance | IR-3.2.3 |
| TC-IR-3.2.3.2 | Material eval reads V-buffer | V-buffer with 3 materials | Correct material applied per pixel | IR-3.2.3 |
| TC-IR-3.2.4.1 | Terrain clipmap renders | 4 LOD levels, camera at center | All levels render, no seams | IR-3.2.4 |
| TC-IR-3.2.4.2 | Virtual texture feedback works | Camera moves to new terrain tile | Correct pages requested and loaded | IR-3.2.4 |
| TC-IR-3.2.5.1 | Foliage GPU cull produces draws | 10K foliage instances, half visible | ~5K in indirect draw args | IR-3.2.5 |
| TC-IR-3.2.6.1 | Ocean FFT pass executes in graph | Ocean entity with 3 cascades | FFT compute pass completes < 1 ms | IR-3.2.6 |
| TC-IR-3.2.6.2 | Sky atmosphere LUT renders | Default atmosphere params | LUT texture valid, no NaN | IR-3.2.6 |
| TC-IR-3.2.7.1 | Missing page uses fallback | Request page not yet streamed | Lowest LOD meshlet rendered | IR-3.2.7 |
| TC-IR-3.2.7.2 | Page load updates GPU buffer | Stream 64 KiB page from disk | GPU buffer updated next frame | IR-3.2.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.2.1.B1 | GPU cull 100K instances | < 1 ms GPU | IR-3.2.1 |
| TC-IR-3.2.2.B1 | LOD resolve 100K instances | < 0.5 ms CPU | IR-3.2.2 |
| TC-IR-3.2.3.B1 | V-buffer + material eval 1080p | < 2 ms GPU | IR-3.2.3 |
| TC-IR-3.2.5.B1 | Foliage cull 1M instances | < 1 ms GPU | IR-3.2.5 |
| TC-IR-3.2.6.B1 | Ocean FFT 3 cascades | < 1 ms GPU | IR-3.2.6 |
| TC-IR-3.2.7.B1 | Page upload 64 KiB | < 0.1 ms CPU | IR-3.2.7 |
