# R-15.5 -- Profiling Tools Requirements

## CPU Profiling

| ID        | Derived From                                               |
|-----------|------------------------------------------------------------|
| R-15.5.1  | [F-15.5.1](../../features/tools-editor/profiling-tools.md) |
| R-15.5.1a | [F-15.5.1](../../features/tools-editor/profiling-tools.md) |

1. **R-15.5.1** — The editor **SHALL** provide a per-frame CPU timeline with swimlane chart, flame
   graph, and flat profile views, filtering by thread or subsystem, and frame-to-frame comparison,
   with measurement overhead under 1% of frame time.
   - **Rationale:** CPU profiling with minimal overhead is essential for identifying bottlenecks
     without distorting measurements.
   - **Verification:** Benchmark: measure profiler overhead at 300+ FPS and verify it stays under
     1%.
2. **R-15.5.1a** — The profiler **SHALL** integrate with ETW on Windows for kernel-level data and
   os_signpost on macOS for Instruments compatibility.
   - **Rationale:** Platform-native profiling APIs provide thread scheduling and hardware counter
     data unavailable through portable instrumentation.
   - **Verification:** Integration test: verify ETW events appear in Windows Performance Analyzer
     and os_signpost events appear in macOS Instruments.

## GPU Profiling

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-15.5.2 | [F-15.5.2](../../features/tools-editor/profiling-tools.md) |

1. **R-15.5.2** — The editor **SHALL** capture GPU timestamps per render graph pass, align the GPU
   timeline with the CPU profiler, and report shader occupancy, wave utilization, and overdraw
   statistics with vendor-specific counters (AMD RGPMT, NVIDIA Nsight, Apple Metal System Trace).
   - **Rationale:** Identifying GPU bottlenecks requires per-pass timing correlated with CPU work.
   - **Verification:** Unit test: verify pass timing sums approximate total GPU frame time within
     10%.

## Memory Profiling

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-15.5.3 | [F-15.5.3](../../features/tools-editor/profiling-tools.md) |
| R-15.5.4 | [F-15.5.4](../../features/tools-editor/profiling-tools.md) |

1. **R-15.5.3** — The editor **SHALL** track all allocations by subsystem and asset type with a live
   treemap, historical timeline, per-frame allocation rate, and per-allocation call-stack capture
   using platform-specific APIs (CaptureStackBackTrace, backtrace).
   - **Rationale:** Memory budget enforcement requires per-subsystem attribution with source-level
     tracing.
   - **Verification:** Unit test: allocate from a known subsystem and verify it attributes correctly
     in the treemap.
2. **R-15.5.4** — The editor **SHALL** detect memory leaks by comparing allocation snapshots,
   reporting leaks grouped by allocation site and asset type, with automated CI leak checks.
   - **Rationale:** Long-running sessions (MMO clients, editor sessions) require automated leak
     detection.
   - **Verification:** Unit test: intentionally leak memory between snapshots and verify the leak is
     detected and reported.

## Network Profiling

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-15.5.5 | [F-15.5.5](../../features/tools-editor/profiling-tools.md) |

1. **R-15.5.5** — The editor **SHALL** provide per-channel bandwidth monitoring, a packet inspector
   decoding individual packets, bandwidth graphing with spike alerts, and per-entity relevance
   breakdown.
   - **Rationale:** Bandwidth budget violations in networked games require per-channel attribution.
   - **Verification:** Unit test: verify per-channel bandwidth sums match total within 1%.

## Stat Overlays

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-15.5.6 | [F-15.5.6](../../features/tools-editor/profiling-tools.md) |

1. **R-15.5.6** — The editor **SHALL** provide individually toggleable viewport overlays for FPS,
   frame time, draw calls, triangle count, GPU memory, and CPU thread utilization, with CSV export
   for post-session analysis.
   - **Rationale:** Real-time stat monitoring enables performance awareness during gameplay testing.
   - **Verification:** Unit test: verify each overlay displays non-zero values during an active
     scene.

## Remote Profiling

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-15.5.7 | [F-15.5.7](../../features/tools-editor/profiling-tools.md) |

1. **R-15.5.7** — The editor **SHALL** support connecting profiling tools to remote servers and
   mobile devices, with all profiler views functioning identically to local data and capture
   bandwidth limited to under 10 Mbps.
   - **Rationale:** Production servers and mobile devices cannot run the full editor; remote
     profiling bridges the gap.
   - **Verification:** Integration test: connect to a remote target and verify profiler data matches
     local profiling for the same workload.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/profiling-tools.md](../../user-stories/tools-editor/profiling-tools.md).
Requirements in this document are derived from those user stories.
