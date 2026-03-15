# R-15.5 — Profiling Tools Requirements

## CPU Profiling

### R-15.5.1 Frame Profiler (CPU Timeline)

The engine **SHALL** capture and display a per-frame CPU timeline showing all job system tasks,
engine subsystem ticks, and visual logic graph execution as color-coded bars on a swimlane chart,
with flame graph and flat profile views, thread/subsystem filtering, and frame-to-frame comparison,
while introducing no more than 1% measurement overhead.

- **Derived from:** [F-15.5.1](../../features/tools-editor/profiling-tools.md)
- **Rationale:** Frame-level CPU profiling is essential for identifying performance bottlenecks in
  gameplay logic, job scheduling, and subsystem ticks across hundreds of FPS.
- **Verification:** Enable the CPU profiler on a scene running at 300+ FPS; measure that average
  frame time increases by no more than 1%. Validate swimlane accuracy by instrumenting a known
  workload with fixed sleep durations and asserting reported durations match within 5% tolerance.

## GPU Profiling

### R-15.5.2 GPU Profiler (Pass Timing and Occupancy)

The engine **SHALL** capture GPU timestamps per render graph pass and present them on a timeline
aligned with the CPU frame profiler, reporting per-pass duration, shader occupancy, wave
utilization, and overdraw statistics, with vendor-specific counter support on AMD (RGPMT),
NVIDIA (Nsight), and Apple (Metal System Trace) GPUs.

- **Derived from:** [F-15.5.2](../../features/tools-editor/profiling-tools.md)
- **Rationale:** GPU pass timing and occupancy data enable artists and engineers to identify
  rendering bottlenecks and optimize shader workloads per target hardware.
- **Verification:** Run a scene with 10+ render passes; verify each pass reports a non-zero
  duration and that the sum of pass durations approximates the total GPU frame time within 10%.
  On each vendor GPU, confirm at least one vendor-specific counter is reported.

## Memory Profiling

### R-15.5.3 Memory Profiler (Allocation Tracking)

The engine **SHALL** track all CPU and GPU memory allocations by subsystem, asset type, and
allocation site, displaying a live treemap, historical timeline, per-frame allocation rate, and
call-stack capture for each allocation.

- **Derived from:** [F-15.5.3](../../features/tools-editor/profiling-tools.md)
- **Rationale:** Detailed allocation tracking is critical for enforcing memory budgets on clients
  that must run for extended sessions without memory growth.
- **Verification:** Allocate 1000 objects from a known subsystem; verify the treemap attributes
  100% of allocations to that subsystem. Validate call stacks resolve to correct source locations
  on Windows (CaptureStackBackTrace) and macOS/Linux (backtrace).

### R-15.5.4 Leak Detection

The engine **SHALL** detect memory leaks by comparing allocation snapshots taken at different points
in time, reporting allocations present in the later snapshot but absent in the earlier one, grouped
by allocation site and asset type, with CI-compatible automated leak checks.

- **Derived from:** [F-15.5.4](../../features/tools-editor/profiling-tools.md)
- **Rationale:** Automated leak detection prevents slow memory growth that degrades stability
  during long play sessions and catches regressions in CI before they ship.
- **Verification:** Intentionally leak 100 allocations between two snapshots; verify the leak
  report lists exactly 100 allocations grouped by their allocation site. Run the same test in
  CI mode and assert a non-zero exit code.

## Network Profiling

### R-15.5.5 Network Profiler (Bandwidth and Packet Inspector)

The engine **SHALL** monitor network bandwidth per replication channel, RPC category, and entity
relevance set, providing a packet inspector that decodes packets into structured fields, graphing
upstream/downstream bandwidth over time, and flagging spikes exceeding budget thresholds.

- **Derived from:** [F-15.5.5](../../features/tools-editor/profiling-tools.md)
- **Rationale:** Network bandwidth profiling is essential for tuning replication in crowded zones
  where bandwidth spikes can cause packet loss and desynchronization.
- **Verification:** Simulate 50 entities replicating across 3 channels; verify per-channel
  bandwidth sums match total bandwidth within 1%. Inject a bandwidth spike and assert the
  threshold alert fires within one sampling interval.

## Non-Functional Requirements

### R-15.5.NF1 Profiler Measurement Overhead

All profiling tools **SHALL** introduce no more than the following overhead when active:
- CPU frame profiler: under 1% of total frame time (as stated in R-15.5.1).
- GPU profiler: under 2% of total GPU frame time from timestamp query insertion.
- Memory profiler with call-stack capture: under 5% of total frame time.
- Network profiler: under 0.5% of total frame time.
- Stat overlays (all enabled): under 0.5ms rendering overhead per frame.

When all profilers are enabled simultaneously, the combined overhead **SHALL NOT** exceed 8% of
total frame time.

- **Derived from:** F-15.5.1 through F-15.5.7 (all profiling features).
- **Rationale:** Profiling tools that significantly perturb the system under measurement produce
  misleading results. Low overhead ensures profiling data reflects real production performance.
- **Verification:** Enable all profilers simultaneously on a scene running at 300+ FPS. Measure
  frame time with and without profiling and assert the difference is under 8%. Measure each
  profiler's individual overhead by enabling them one at a time and assert each stays within its
  specified budget.

## Overlays and Remote Profiling

### R-15.5.6 Stat Overlays

The engine **SHALL** render configurable HUD overlays on the game viewport showing real-time FPS,
frame time, draw call count, triangle count, GPU memory, CPU thread utilization, network bandwidth,
and entity count, with individual toggle control and CSV export for post-session analysis.

- **Derived from:** [F-15.5.6](../../features/tools-editor/profiling-tools.md)
- **Rationale:** On-screen stat overlays give immediate visual feedback during play testing
  without switching to external profiling tools.
- **Verification:** Enable all overlays; verify each displays a non-zero value during an active
  scene. Toggle each overlay off individually and confirm it disappears. Export to CSV and
  validate the file parses correctly with at least 60 rows of data for a 1-second capture.

### R-15.5.7 Remote Profiling

The engine **SHALL** connect editor-side profiling tools to a live game server or client over TCP
for remote data collection, with all profiler views (CPU timeline, GPU passes, memory, network)
functioning identically when driven by remote data, supporting dedicated servers, development kits,
and mobile devices, while limiting capture bandwidth to under 10 Mbps.

- **Derived from:** [F-15.5.7](../../features/tools-editor/profiling-tools.md)
- **Rationale:** Remote profiling enables performance analysis of dedicated servers under real
  player load and mobile devices that lack local profiling UI.
- **Verification:** Connect the editor profiler to a remote game instance on the local network;
  verify CPU timeline, GPU pass, memory, and network views display valid data. Measure capture
  stream bandwidth and assert it remains below 10 Mbps at default granularity.
