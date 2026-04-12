# Audio ↔ Spatial Awareness Integration Test Cases

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

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.9.1.B1 | 100 sources x 100 rays | < 2 ms | IR-1.9.1 |
| TC-IR-1.9.5.B1 | 25 sources traced/frame | < 0.5 ms | IR-1.9.5 |
| TC-IR-1.9.3.B1 | Double-buffer swap | < 0.001 ms | IR-1.9.3 |
