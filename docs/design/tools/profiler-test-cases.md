# Profiling Tools Test Cases

Companion test cases for [profiler.md](profiler.md).

## Unit Tests

### TC-15.5.1.1 Ring Buffer Push and Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push 10,000 `CpuEvent` entries, drain all | drained count == 10,000, no events lost | R-15.5.1 |
| 2 | Push 0 events, drain | drained count == 0 | R-15.5.1 |

### TC-15.5.1.2 Ring Buffer Overflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push `capacity + 1` events without draining | `events_dropped() == true` | R-15.5.1 |
| 2 | Push `capacity - 1` events without draining | `events_dropped() == false` | R-15.5.1 |

### TC-15.5.1.3 Frame Collector Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push events from 4 threads with interleaved timestamps, `collect_frame()` | `cpu_events` sorted by `begin_tsc` ascending | R-15.5.1 |

### TC-15.5.1.4 Flame Graph Depth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Nested scopes: A { B { C } } | events have `depth` values 0, 1, 2 respectively | R-15.5.1 |

### TC-15.5.1.5 Timeline Filter by Thread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capture with events from threads 1,2,3; filter `thread_ids: [2]` | only thread 2 events displayed | R-15.5.1 |
| 2 | Filter `thread_ids: None` (no filter) | all thread events displayed | R-15.5.1 |

### TC-15.5.1.6 Frame Comparison

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Frame A: system_X = 5ms; Frame B: system_X = 8ms; `set_comparison(A, B)` | delta for system_X shows +3ms | R-15.5.1 |

### TC-15.5.2.1 GPU Pass Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert begin/end timestamp queries around a render pass | `GpuPassTiming.duration_ms > 0.0` | R-15.5.2 |

### TC-15.5.2.2 GPU-CPU Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capture GPU and CPU events for same frame | GPU pass `begin_ms` falls within CPU frame time range | R-15.5.2 |

### TC-15.5.2.3 Vendor Counter (AMD)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query `shader_occupancy()` on AMD GPU | `Some(value)` where `0.0 <= value <= 1.0` | R-15.5.2 |
| 2 | Query `shader_occupancy()` on non-AMD GPU | `None` | R-15.5.2 |

### TC-15.5.3.1 Allocation Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `record_alloc` 100 blocks of varying sizes | `take_snapshot().allocations.len() == 100`, sizes match | R-15.5.3 |

### TC-15.5.3.2 Deallocation Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Alloc 50 blocks, dealloc 20, `take_snapshot()` | `allocations.len() == 30` | R-15.5.3 |

### TC-15.5.3.3 Treemap by Subsystem

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocs tagged with subsystems A, B, C; `memory_by_subsystem()` | sum of per-subsystem bytes == `total_bytes` | R-15.5.3 |

### TC-15.5.3.4 Call Stack Capture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `StackCapture::capture(16)` from a known 4-deep call chain | `len() >= 3`, contains known function addresses | R-15.5.3 |

### TC-15.5.3.5 Per-Frame Allocation Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate 500 blocks in one frame, query `per_frame_alloc_rate()` | returns 500 | R-15.5.3 |

### TC-15.5.4.1 Leak Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Baseline snapshot, alloc 10 blocks without free, comparison snapshot | `LeakReport.total_leaked_count == 10` | R-15.5.4 |

### TC-15.5.4.2 No False Leak

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Baseline snapshot, alloc 10 blocks then free all 10, comparison snapshot | `LeakReport.total_leaked_count == 0` | R-15.5.4 |

### TC-15.5.4.3 Leak Grouping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 leaks from call site A, 3 from call site B | 2 `LeakGroup` entries with counts 5 and 3 | R-15.5.4 |

### TC-15.5.5.1 Network Bandwidth per Channel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 100 bytes on channel 1, 200 bytes on channel 2, 300 bytes on channel 3 | `per_channel_bandwidth()` returns 3 entries with correct byte counts | R-15.5.5 |

### TC-15.5.5.2 Network Bandwidth Sum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record on 3 channels, sum per-channel totals | sum within 1% of `FrameStats.net_bandwidth_bps` | R-15.5.5 |

### TC-15.5.5.3 Packet Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encode known packet with 3 fields, `decode()` result | `DecodedPacket.fields.len() == 3`, field names and values match | R-15.5.5 |

### TC-15.5.6.1 Overlay FPS Non-Zero

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Active scene rendering, read FPS overlay value | FPS value > 0 | R-15.5.6 |

### TC-15.5.6.2 Overlay Toggle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_enabled(Fps, false)` | FPS overlay not rendered | R-15.5.6 |
| 2 | `set_enabled(Fps, true)` | FPS overlay rendered | R-15.5.6 |

### TC-15.5.6.3 CSV Export

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 10 frames, `start_csv_recording`, `stop_csv_recording` | CSV file contains 10 data rows | R-15.5.6 |

### TC-15.5.7.1 Remote Encode-Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `BinaryProtocol::encode(capture)`, then `decode(bytes)` | decoded `FrameCapture` identical to original | R-15.5.7 |
| 2 | `decode` truncated bytes | `Err(DecodeError::TruncatedData)` | R-15.5.7 |

### TC-15.5.7.2 Remote Connect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start `RemoteServer` on port 9000, `RemoteClient::connect("localhost", 9000)` | `Ok(())`, `is_connected() == true` | R-15.5.7 |
| 2 | `connect` to non-listening port | `Err(RemoteError::ConnectionRefused)` | R-15.5.7 |

### TC-15.5.1.7 ECS System Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `record_system("physics", 150.0)`, `system_timings()` | contains entry with `name == "physics"`, `duration_us == 150.0` | R-15.5.1 |
| 2 | Record 10 systems, `top_systems(3)` | returns 3 entries sorted by `duration_us` descending | R-15.5.1 |

## Integration Tests

### TC-15.5.1.I1 Overhead Under 1 Percent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run 300 FPS workload with profiler enabled vs disabled | overhead < 1% of frame time | R-15.5.1 |

### TC-15.5.1.I2 ETW Integration (Windows)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable `EtwIntegration`, run 100 frames | `context_switches()` returns non-empty list | R-15.5.1 |

### TC-15.5.1.I3 Signpost Integration (macOS)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable `SignpostIntegration`, emit begin/end | Instruments captures signpost events | R-15.5.1 |

### TC-15.5.1.I4 Perf Integration (Linux)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable `PerfIntegration`, `read_hardware_counters()` | returns `HardwareCounters` with non-zero values | R-15.5.1 |

### TC-15.5.2.I1 GPU Pass Duration Sum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sum all `GpuPassTiming.duration_ms` | within 10% of `FrameStats.gpu_frame_ms` | R-15.5.2 |

### TC-15.5.7.I1 Remote Data Fidelity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Profile same workload locally and remotely | remote `FrameCapture` matches local within 1% for all metrics | R-15.5.7 |

### TC-15.5.7.I2 Remote Bandwidth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream at `Full` granularity for 60 seconds | `bandwidth_usage() < 10_000_000` (10 Mbps) | R-15.5.7 |

### TC-15.5.4.I1 Leak CI Automation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run scenario with intentional leak, `assert_no_leaks()` | `Err(LeakReport)` with correct leak count | R-15.5.4 |
| 2 | Run clean scenario, `assert_no_leaks()` | `Ok(())` | R-15.5.4 |

### TC-15.5.6.I1 Overlay All Platforms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable all overlays on Windows, macOS, Linux | all overlays render correctly on each platform | R-15.5.6 |

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
