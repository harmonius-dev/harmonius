# Profiling Tools Test Cases

Companion test cases for [profiler.md](profiler.md).

## Unit Tests

### TC-15.5.1.1 Ring Buffer Push and Drain

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |
| 2 | R-15.5.1    |

1. **#1** — Push 10,000 `CpuEvent` entries, drain all
   - **Expected:** drained count == 10,000, no events lost
2. **#2** — Push 0 events, drain
   - **Expected:** drained count == 0

### TC-15.5.1.2 Ring Buffer Overflow

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |
| 2 | R-15.5.1    |

1. **#1** — Push `capacity + 1` events without draining
   - **Expected:** `events_dropped() == true`
2. **#2** — Push `capacity - 1` events without draining
   - **Expected:** `events_dropped() == false`

### TC-15.5.1.3 Frame Collector Sort

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Push events from 4 threads with interleaved timestamps, `collect_frame()`
   - **Expected:** `cpu_events` sorted by `begin_tsc` ascending

### TC-15.5.1.4 Flame Graph Depth

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Nested scopes: A { B { C } }
   - **Expected:** events have `depth` values 0, 1, 2 respectively

### TC-15.5.1.5 Timeline Filter by Thread

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |
| 2 | R-15.5.1    |

1. **#1** — Capture with events from threads 1,2,3; filter `thread_ids: [2]`
   - **Expected:** only thread 2 events displayed
2. **#2** — Filter `thread_ids: None` (no filter)
   - **Expected:** all thread events displayed

### TC-15.5.1.6 Frame Comparison

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Frame A: system_X = 5ms; Frame B: system_X = 8ms; `set_comparison(A, B)`
   - **Expected:** delta for system_X shows +3ms

### TC-15.5.2.1 GPU Pass Timing

| # | Requirement |
|---|-------------|
| 1 | R-15.5.2    |

1. **#1** — Insert begin/end timestamp queries around a render pass
   - **Expected:** `GpuPassTiming.duration_ms > 0.0`

### TC-15.5.2.2 GPU-CPU Alignment

| # | Requirement |
|---|-------------|
| 1 | R-15.5.2    |

1. **#1** — Capture GPU and CPU events for same frame
   - **Expected:** GPU pass `begin_ms` falls within CPU frame time range

### TC-15.5.2.3 Vendor Counter (AMD)

| # | Requirement |
|---|-------------|
| 1 | R-15.5.2    |
| 2 | R-15.5.2    |

1. **#1** — Query `shader_occupancy()` on AMD GPU
   - **Expected:** `Some(value)` where `0.0 <= value <= 1.0`
2. **#2** — Query `shader_occupancy()` on non-AMD GPU
   - **Expected:** `None`

### TC-15.5.3.1 Allocation Tracking

| # | Requirement |
|---|-------------|
| 1 | R-15.5.3    |

1. **#1** — `record_alloc` 100 blocks of varying sizes
   - **Expected:** `take_snapshot().allocations.len() == 100`, sizes match

### TC-15.5.3.2 Deallocation Tracking

| # | Requirement |
|---|-------------|
| 1 | R-15.5.3    |

1. **#1** — Alloc 50 blocks, dealloc 20, `take_snapshot()`
   - **Expected:** `allocations.len() == 30`

### TC-15.5.3.3 Treemap by Subsystem

| # | Requirement |
|---|-------------|
| 1 | R-15.5.3    |

1. **#1** — Allocs tagged with subsystems A, B, C; `memory_by_subsystem()`
   - **Expected:** sum of per-subsystem bytes == `total_bytes`

### TC-15.5.3.4 Call Stack Capture

| # | Requirement |
|---|-------------|
| 1 | R-15.5.3    |

1. **#1** — `StackCapture::capture(16)` from a known 4-deep call chain
   - **Expected:** `len() >= 3`, contains known function addresses

### TC-15.5.3.5 Per-Frame Allocation Rate

| # | Requirement |
|---|-------------|
| 1 | R-15.5.3    |

1. **#1** — Allocate 500 blocks in one frame, query `per_frame_alloc_rate()`
   - **Expected:** returns 500

### TC-15.5.4.1 Leak Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.5.4    |

1. **#1** — Baseline snapshot, alloc 10 blocks without free, comparison snapshot
   - **Expected:** `LeakReport.total_leaked_count == 10`

### TC-15.5.4.2 No False Leak

| # | Requirement |
|---|-------------|
| 1 | R-15.5.4    |

1. **#1** — Baseline snapshot, alloc 10 blocks then free all 10, comparison snapshot
   - **Expected:** `LeakReport.total_leaked_count == 0`

### TC-15.5.4.3 Leak Grouping

| # | Requirement |
|---|-------------|
| 1 | R-15.5.4    |

1. **#1** — 5 leaks from call site A, 3 from call site B
   - **Expected:** 2 `LeakGroup` entries with counts 5 and 3

### TC-15.5.5.1 Network Bandwidth per Channel

| # | Requirement |
|---|-------------|
| 1 | R-15.5.5    |

1. **#1** — Record 100 bytes on channel 1, 200 bytes on channel 2, 300 bytes on channel 3
   - **Expected:** `per_channel_bandwidth()` returns 3 entries with correct byte counts

### TC-15.5.5.2 Network Bandwidth Sum

| # | Requirement |
|---|-------------|
| 1 | R-15.5.5    |

1. **#1** — Record on 3 channels, sum per-channel totals
   - **Expected:** sum within 1% of `FrameStats.net_bandwidth_bps`

### TC-15.5.5.3 Packet Decode

| # | Requirement |
|---|-------------|
| 1 | R-15.5.5    |

1. **#1** — Encode known packet with 3 fields, `decode()` result
   - **Expected:** `DecodedPacket.fields.len() == 3`, field names and values match

### TC-15.5.6.1 Overlay FPS Non-Zero

| # | Requirement |
|---|-------------|
| 1 | R-15.5.6    |

1. **#1** — Active scene rendering, read FPS overlay value
   - **Expected:** FPS value > 0

### TC-15.5.6.2 Overlay Toggle

| # | Requirement |
|---|-------------|
| 1 | R-15.5.6    |
| 2 | R-15.5.6    |

1. **#1** — `set_enabled(Fps, false)`
   - **Expected:** FPS overlay not rendered
2. **#2** — `set_enabled(Fps, true)`
   - **Expected:** FPS overlay rendered

### TC-15.5.6.3 CSV Export

| # | Requirement |
|---|-------------|
| 1 | R-15.5.6    |

1. **#1** — Record 10 frames, `start_csv_recording`, `stop_csv_recording`
   - **Expected:** CSV file contains 10 data rows

### TC-15.5.7.1 Remote Encode-Decode

| # | Requirement |
|---|-------------|
| 1 | R-15.5.7    |
| 2 | R-15.5.7    |

1. **#1** — `BinaryProtocol::encode(capture)`, then `decode(bytes)`
   - **Expected:** decoded `FrameCapture` identical to original
2. **#2** — `decode` truncated bytes
   - **Expected:** `Err(DecodeError::TruncatedData)`

### TC-15.5.7.2 Remote Connect

| # | Requirement |
|---|-------------|
| 1 | R-15.5.7    |
| 2 | R-15.5.7    |

1. **#1** — Start `RemoteServer` on port 9000, `RemoteClient::connect("localhost", 9000)`
   - **Expected:** `Ok(())`, `is_connected() == true`
2. **#2** — `connect` to non-listening port
   - **Expected:** `Err(RemoteError::ConnectionRefused)`

### TC-15.5.1.7 ECS System Timing

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |
| 2 | R-15.5.1    |

1. **#1** — `record_system("physics", 150.0)`, `system_timings()`
   - **Expected:** contains entry with `name == "physics"`, `duration_us == 150.0`
2. **#2** — Record 10 systems, `top_systems(3)`
   - **Expected:** returns 3 entries sorted by `duration_us` descending

## Integration Tests

### TC-15.5.1.I1 Overhead Under 1 Percent

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Run 300 FPS workload with profiler enabled vs disabled
   - **Expected:** overhead < 1% of frame time

### TC-15.5.1.I2 ETW Integration (Windows)

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Enable `EtwIntegration`, run 100 frames
   - **Expected:** `context_switches()` returns non-empty list

### TC-15.5.1.I3 Signpost Integration (macOS)

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Enable `SignpostIntegration`, emit begin/end
   - **Expected:** Instruments captures signpost events

### TC-15.5.1.I4 Perf Integration (Linux)

| # | Requirement |
|---|-------------|
| 1 | R-15.5.1    |

1. **#1** — Enable `PerfIntegration`, `read_hardware_counters()`
   - **Expected:** returns `HardwareCounters` with non-zero values

### TC-15.5.2.I1 GPU Pass Duration Sum

| # | Requirement |
|---|-------------|
| 1 | R-15.5.2    |

1. **#1** — Sum all `GpuPassTiming.duration_ms`
   - **Expected:** within 10% of `FrameStats.gpu_frame_ms`

### TC-15.5.7.I1 Remote Data Fidelity

| # | Requirement |
|---|-------------|
| 1 | R-15.5.7    |

1. **#1** — Profile same workload locally and remotely
   - **Expected:** remote `FrameCapture` matches local within 1% for all metrics

### TC-15.5.7.I2 Remote Bandwidth

| # | Requirement |
|---|-------------|
| 1 | R-15.5.7    |

1. **#1** — Stream at `Full` granularity for 60 seconds
   - **Expected:** `bandwidth_usage() < 10_000_000` (10 Mbps)

### TC-15.5.4.I1 Leak CI Automation

| # | Requirement |
|---|-------------|
| 1 | R-15.5.4    |
| 2 | R-15.5.4    |

1. **#1** — Run scenario with intentional leak, `assert_no_leaks()`
   - **Expected:** `Err(LeakReport)` with correct leak count
2. **#2** — Run clean scenario, `assert_no_leaks()`
   - **Expected:** `Ok(())`

### TC-15.5.6.I1 Overlay All Platforms

| # | Requirement |
|---|-------------|
| 1 | R-15.5.6    |

1. **#1** — Enable all overlays on Windows, macOS, Linux
   - **Expected:** all overlays render correctly on each platform

### TC-15.5.1.I5 CPU event capture end-to-end

| # | Requirement |
|---|-------------|
| 1 | US-15.5.1   |

1. **#1** — Enable CPU profiler, run a 60-frame workload with instrumented scopes, stop, export.
   - **Expected:** Export contains all scope events with thread id, parent id, and start/end
     timestamps; no events dropped under instrumentation budget.

### TC-15.5.3.I1 Allocation recording with call stacks

| # | Requirement |
|---|-------------|
| 1 | US-15.5.3   |

1. **#1** — Record allocations for a 10 s workload; snapshot start and end.
   - **Expected:** Snapshot diff shows each allocation with stack depth >= 8 frames; totals match
     the test harness's counter.

### TC-15.5.4.I1 Leak detection across snapshots

| # | Requirement |
|---|-------------|
| 1 | US-15.5.4   |

1. **#1** — Leak 1000 known allocations between snapshot A and snapshot B; run leak diff.
   - **Expected:** Report lists exactly the leaked allocations with correct sizes; CI exit code
     non-zero.

### TC-15.5.7.I3 Remote capture session

| # | Requirement |
|---|-------------|
| 1 | US-15.5.7   |

1. **#1** — Connect remote profiler client to running game; capture a 5 s session over the wire.
   - **Expected:** Client receives complete frame timeline; data fidelity matches local capture;
     bandwidth within budget.

## Benchmarks

### TC-15.5.1.B1 CpuScope Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `CpuScope::begin()` + `end()` in tight loop | latency per pair | < 20 ns | R-15.5.1 |

### TC-15.5.1.B2 Ring Buffer Push

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `push()` single event in tight loop | latency per push | < 5 ns | R-15.5.1 |

### TC-15.5.1.B3 Frame Drain

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Drain 1000 events from ring buffer | latency | < 50 us | R-15.5.1 |

### TC-15.5.1.B4 Total Overhead at 300 FPS

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 CPU events/frame at 300 FPS | overhead % of frame time | < 1% | R-15.5.1 |

### TC-15.5.3.B1 Allocation Record

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `record_alloc` without stack capture | latency | < 50 ns | R-15.5.3 |

### TC-15.5.3.B2 Stack Capture

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `StackCapture::capture(16)` | latency | < 1 us | R-15.5.3 |

### TC-15.5.4.B1 Snapshot Diff

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Diff two snapshots with 100k allocations each | latency | < 100 ms | R-15.5.4 |

### TC-15.5.7.B1 Remote Encode

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `BinaryProtocol::encode` for 1 frame (1000 events) | latency | < 500 us | R-15.5.7 |

### TC-15.5.7.B2 Remote Bandwidth

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Stream at `Full` granularity | bandwidth | < 10 Mbps | R-15.5.7 |

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-15.5.1.5
- US-15.5.1.8
- US-15.5.3.5
- US-15.5.3.6
- US-15.5.4.1
- US-15.5.7.5
