# Profiler ↔ Game Loop Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.6.1.1 | Phase 1 Input timed | Run 1 frame with input | CpuEvent for "Phase1_Input" in capture | IR-5.6.1 |
| TC-IR-5.6.1.2 | All 8 phases timed | Run 1 frame | 8 phase CpuEvents in FrameCapture | IR-5.6.1 |
| TC-IR-5.6.1.3 | Nested system scopes | Phase with 3 systems | 3 child CpuEvents under phase event | IR-5.6.1 |
| TC-IR-5.6.2.1 | Frame budget breakdown | 16.6 ms frame target | Per-phase percentages sum to 100% | IR-5.6.2 |
| TC-IR-5.6.2.2 | Budget tracks over time | 100 frames | Rolling average converges to stable value | IR-5.6.2 |
| TC-IR-5.6.3.1 | Spike detected on slow phase | Phase 5 takes 10 ms (budget 4 ms) | SpikeEntry for Phase5_Physics | IR-5.6.3 |
| TC-IR-5.6.3.2 | No spike when within budget | All phases under budget | Empty spike list | IR-5.6.3 |
| TC-IR-5.6.3.3 | Spike includes frame number | Spike on frame 42 | SpikeEntry.frame_number == 42 | IR-5.6.3 |
| TC-IR-5.6.4.1 | Ring buffer drained at boundary | 50 events across 4 threads | All 50 events in FrameCapture | IR-5.6.4 |
| TC-IR-5.6.4.2 | Events sorted by timestamp | Events from multiple threads | FrameCapture.cpu_events sorted ascending | IR-5.6.4 |
| TC-IR-5.6.5.1 | Per-system timing recorded | 10 ECS systems in Phase 3 | 10 CpuEvents with system names | IR-5.6.5 |
| TC-IR-5.6.5.2 | System timing matches wall clock | Busy-wait system 1 ms | CpuEvent duration within 0.1 ms of 1 ms | IR-5.6.5 |
| TC-IR-5.6.6.1 | Substep profiling | 4 physics substeps | 4 CpuEvents for "Physics_Substep" | IR-5.6.6 |
| TC-IR-5.6.6.2 | Substep durations sum correctly | 4 substeps, each ~1 ms | Phase 5 total ~4 ms | IR-5.6.6 |
| TC-IR-5.6.7.1 | StatOverlay reads FrameStats | Run 1 frame | Overlay displays FPS and frame time | IR-5.6.7 |
| TC-IR-5.6.7.2 | FrameStats entity count | World with 500 entities | FrameStats.entity_count == 500 | IR-5.6.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.6.1.B1 | CpuScopeGuard cost | < 20 ns begin+end | IR-5.6.1 |
| TC-IR-5.6.4.B1 | FrameCollector drain 1000 events | < 0.1 ms | IR-5.6.4 |
| TC-IR-5.6.4.B2 | Profiler total overhead per frame | < 1% of 16.6 ms | IR-5.6.4 |
| TC-IR-5.6.5.B1 | EcsSystemTracker per-system cost | < 50 ns | IR-5.6.5 |
