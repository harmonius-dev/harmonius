# Profiler ↔ Rendering Integration Test Cases

All tests run on the headless `FakeGpuBackend` in `harmonius_gpu_runtime_test` so CI has no GPU
vendor dependency. Tests that assert vendor-specific behaviour are gated behind a skip on missing
capability, not a compile flag.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.7.1.U1 | `GpuScope::drop` emits end | 1 pass, enabled | begin+end slot paired | IR-5.7.1 |
| TC-IR-5.7.1.U2 | Flat-slot name lookup | 128 slots | O(1) index-into-Vec lookup | IR-5.7.1 |
| TC-IR-5.7.2.U1 | `DrawList::stats()` sums | 10 cmds, 300 tris | DrawListStats matches | IR-5.7.2 |
| TC-IR-5.7.4.U1 | Timebase calibration | 1 anchor sample | ms == ticks / gpu_hz | IR-5.7.4 |
| TC-IR-5.7.7.U1 | `enabled=false` early-out | 1 pass, disabled | No slot allocated | IR-5.7.7 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.7.1.1 | Shadow pass timed | Scene 1 dir light | GpuPassTiming("ShadowPass") | IR-5.7.1 |
| TC-IR-5.7.1.2 | All passes timed | 8 passes one frame | 8 GpuPassTiming entries | IR-5.7.1 |
| TC-IR-5.7.1.3 | Durations non-negative | 100 frames | all duration_ms >= 0.0 | IR-5.7.1 |
| TC-IR-5.7.2.1 | Draw call count | 50 opaque + 10 transp | draw_calls == 60 | IR-5.7.2 |
| TC-IR-5.7.2.2 | Triangle count | mesh 1000 tris | triangles == 1000 | IR-5.7.2 |
| TC-IR-5.7.2.3 | Meshlet cull ratio | 1000 meshlets, 50% cull | meshlets_culled ~ 500 | IR-5.7.2 |
| TC-IR-5.7.3.1 | VRAM textures | 10 tex, 100 MB | vram_textures >= 100 MB | IR-5.7.3 |
| TC-IR-5.7.3.2 | VRAM buffers | 50 MB mesh upload | vram_buffers >= 50 MB | IR-5.7.3 |
| TC-IR-5.7.3.3 | VRAM render targets | 4 RTs allocated | vram_render_targets > 0 | IR-5.7.3 |
| TC-IR-5.7.4.1 | GPU after CPU submit | 1 frame | GPU begin_ms > CPU submit | IR-5.7.4 |
| TC-IR-5.7.4.2 | 2-frame readback | capture N, read N+2 | frame N available at N+2 | IR-5.7.4 |
| TC-IR-5.7.5.1 | Timeline labels | 5 named passes | 5 labeled bars | IR-5.7.5 |
| TC-IR-5.7.5.2 | Timing sum | 8 pass timings | sum within 10% of total | IR-5.7.5 |
| TC-IR-5.7.6.1 | Per-view stats | main + shadow cams | Separate DrawListStats | IR-5.7.6 |
| TC-IR-5.7.7.1 | Runtime toggle off | enabled=false | 0 query allocations | IR-5.7.7 |
| TC-IR-5.7.7.2 | Runtime toggle on | enabled=true | Queries allocated | IR-5.7.7 |
| TC-IR-5.7.7.3 | Toggle mid-run | false->true->false | Partial capture OK | IR-5.7.7 |
| TC-IR-5.7.F.1 | MPSC handoff | 1 producer, 1 worker | ResolvedTimestamps drained | IR-5.7.4 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.7.1.N1 | Pool exhausted | cap=4, 5 passes | allocate_pair returns None | IR-5.7.1 |
| TC-IR-5.7.1.N2 | Pass panics | panic between new+drop | end query still emitted | IR-5.7.1 |
| TC-IR-5.7.1.N3 | Unpaired query | missing end slot | skipped on read, warn log | IR-5.7.1 |
| TC-IR-5.7.1.N4 | Pool grow path | None -> grow -> retry | capacity doubles next frame | IR-5.7.1 |
| TC-IR-5.7.2.N1 | Zero draws | empty DrawList | draw_calls==0, triangles==0 | IR-5.7.2 |
| TC-IR-5.7.3.N1 | VRAM over budget | allocate beyond cap | stats capped, error raised | IR-5.7.3 |
| TC-IR-5.7.4.N1 | Timestamp overflow | force wrap | delta re-anchors | IR-5.7.4 |
| TC-IR-5.7.4.N2 | Readback not ready | long frame | Vec empty, reuse prior cap | IR-5.7.4 |
| TC-IR-5.7.5.N1 | Zero passes | frame w/ no graph | empty timings, no panic | IR-5.7.5 |
| TC-IR-5.7.6.N1 | Zero views | no cameras | empty per_view, ok | IR-5.7.6 |
| TC-IR-5.7.7.N1 | MPSC full | cap=4, 5 sends | oldest dropped, log once | IR-5.7.7 |
| TC-IR-5.7.7.N2 | Toggle off frees | enabled=false 60s | pool reset, no leak | IR-5.7.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.7.1.B1 | `GpuScope` new+drop CPU | < 0.5 us | IR-5.7.1 |
| TC-IR-5.7.1.B2 | `GpuScope` GPU cost | < 1 us | IR-5.7.1 |
| TC-IR-5.7.2.B1 | Stat collection per frame | < 0.05 ms CPU | IR-5.7.2 |
| TC-IR-5.7.4.B1 | `read_resolved` 128 pairs | < 50 us | IR-5.7.4 |
| TC-IR-5.7.4.B2 | Readback latency | <= 2 frames | IR-5.7.4 |
| TC-IR-5.7.7.B1 | MPSC send p99 | < 5 us | IR-5.7.7 |
| TC-IR-5.7.7.B2 | Toggle-off overhead | 0 ms amortized | IR-5.7.7 |
| TC-IR-5.7.7.B3 | FrameCollector::assemble_gpu | < 0.1 ms worker | IR-5.7.7 |
