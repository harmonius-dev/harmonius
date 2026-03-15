# R-15.5 -- Profiling Tools User Stories

## US-15.5.1 Frame Profiler (CPU Timeline)

### US-15.5.1.1
As a **developer (P-15)**, I want a per-frame CPU timeline with swimlane chart so that I can see all
job system tasks and subsystem ticks per frame.

### US-15.5.1.2
As a **developer (P-15)**, I want flame graph and flat profile views so that I can analyze call
depth and flat time distribution.

### US-15.5.1.3
As a **developer (P-15)**, I want filtering by thread or subsystem so that I can isolate specific
systems for targeted analysis.

### US-15.5.1.4
As a **developer (P-15)**, I want frame-to-frame comparison so that I can identify regressions by
comparing two captures.

### US-15.5.1.5
As an **engine dev (P-26)**, I want measurement overhead under 1% of frame time so that profiling
data reflects real production performance.

### US-15.5.1.6
As an **engine dev (P-26)**, I want ETW integration on Windows for kernel-level data so that thread
scheduling information supplements the CPU timeline.

### US-15.5.1.7
As an **engine dev (P-26)**, I want os_signpost integration on macOS so that profiler data is
compatible with Instruments for deep analysis.

### US-15.5.1.8
As an **engine tester (P-27)**, I want to verify profiler overhead stays under 1% at 300+ FPS so
that the overhead target is regression-tested under high frame rates.

### US-15.5.1.9
As a **technical artist (P-13)**, I want to profile logic graph execution time so that I can
identify expensive graph nodes affecting performance.

---

## US-15.5.2 GPU Profiler (Pass Timing and Occupancy)

### US-15.5.2.1
As a **developer (P-15)**, I want GPU timestamps per render graph pass so that I can identify which
render passes consume the most GPU time.

### US-15.5.2.2
As a **developer (P-15)**, I want the GPU timeline aligned with the CPU profiler so that I can
correlate CPU and GPU work within the same frame.

### US-15.5.2.3
As a **developer (P-15)**, I want shader occupancy and wave utilization metrics so that I can
diagnose underutilized GPU shaders.

### US-15.5.2.4
As a **developer (P-15)**, I want overdraw statistics per pass so that I can identify scenes with
excessive pixel re-rendering.

### US-15.5.2.5
As a **artist (P-8)**, I want to see per-pass GPU timing on my materials so that I can understand
which material changes impact GPU performance.

### US-15.5.2.6
As an **engine dev (P-26)**, I want vendor-specific counters (AMD RGPMT, NVIDIA Nsight, Apple Metal
System Trace) so that I can access hardware-specific performance data.

### US-15.5.2.7
As an **engine tester (P-27)**, I want to verify pass timing sums approximate total GPU frame time
within 10% so that GPU profiler accuracy is regression-tested.

---

## US-15.5.3 Memory Profiler (Allocation Tracking)

### US-15.5.3.1
As a **developer (P-15)**, I want allocation tracking by subsystem and asset type so that I can
identify which systems consume the most memory.

### US-15.5.3.2
As a **developer (P-15)**, I want a live treemap of memory consumption so that I can visualize
memory distribution interactively.

### US-15.5.3.3
As a **developer (P-15)**, I want historical allocation timeline so that I can see memory growth
patterns over time.

### US-15.5.3.4
As a **developer (P-15)**, I want per-frame allocation rate display so that I can identify frames
with excessive allocations.

### US-15.5.3.5
As a **developer (P-15)**, I want call-stack capture for each allocation so that I can trace
allocations to their source code location.

### US-15.5.3.6
As an **engine dev (P-26)**, I want platform-specific stack capture APIs used
(CaptureStackBackTrace, backtrace) so that call stacks resolve correctly on each platform.

### US-15.5.3.7
As an **engine tester (P-27)**, I want to verify all allocations from a known subsystem attribute to
that subsystem in the treemap so that allocation attribution is regression-tested.

---

## US-15.5.4 Leak Detection

### US-15.5.4.1
As a **developer (P-15)**, I want to compare allocation snapshots for leak detection so that I can
find allocations that persist between checkpoints.

### US-15.5.4.2
As a **developer (P-15)**, I want leak reports grouped by allocation site and asset type so that I
can prioritize fixes by impact.

### US-15.5.4.3
As a **DevOps (P-16)**, I want automated leak checks in CI so that memory leaks are caught before
they reach production.

### US-15.5.4.4
As an **engine tester (P-27)**, I want to verify intentional leaks are detected between snapshots so
that leak detection accuracy is regression-tested.

---

## US-15.5.5 Network Profiler (Bandwidth and Packet Inspector)

### US-15.5.5.1
As a **developer (P-15)**, I want bandwidth monitoring per replication channel so that I can
identify which data channels consume the most bandwidth.

### US-15.5.5.2
As a **developer (P-15)**, I want a packet inspector decoding individual packets so that I can debug
serialization issues at the field level.

### US-15.5.5.3
As a **developer (P-15)**, I want bandwidth graphing over time with spike alerts so that I can
identify bandwidth budget violations.

### US-15.5.5.4
As a **server admin (P-22)**, I want per-entity-relevance bandwidth breakdown so that I can tune
replication for crowded zones.

### US-15.5.5.5
As an **engine tester (P-27)**, I want to verify per-channel bandwidth sums match total within 1% so
that bandwidth accounting is regression-tested.

---

## US-15.5.6 Stat Overlays

### US-15.5.6.1
As a **designer (P-5)**, I want FPS and frame time overlays on the game viewport so that I can
monitor performance during play testing.

### US-15.5.6.2
As a **developer (P-15)**, I want draw call count and triangle count overlays so that I can monitor
rendering complexity during gameplay.

### US-15.5.6.3
As a **developer (P-15)**, I want GPU memory and CPU thread utilization overlays so that I can watch
resource usage in real time.

### US-15.5.6.4
As a **developer (P-15)**, I want overlays individually toggleable so that I can show only the stats
relevant to my current task.

### US-15.5.6.5
As a **developer (P-15)**, I want CSV export of overlay data for post-session analysis so that I can
process performance data in external tools.

### US-15.5.6.6
As an **engine tester (P-27)**, I want to verify each overlay displays non-zero values during an
active scene so that overlay data accuracy is regression-tested.

---

## US-15.5.7 Remote Profiling

### US-15.5.7.1
As a **developer (P-15)**, I want to connect editor profiling tools to a remote server so that I can
profile dedicated servers under real player load.

### US-15.5.7.2
As a **developer (P-15)**, I want all profiler views to function identically with remote data so
that remote profiling provides the same insights as local profiling.

### US-15.5.7.3
As a **developer (P-15)**, I want remote profiling of mobile devices on the local network so that I
can analyze mobile performance without on-device tools.

### US-15.5.7.4
As a **server admin (P-22)**, I want to connect profiling tools to live game servers so that I can
diagnose production performance issues.

### US-15.5.7.5
As an **engine dev (P-26)**, I want capture bandwidth limited to under 10 Mbps so that profiling
does not saturate the network connection.

### US-15.5.7.6
As an **engine tester (P-27)**, I want to verify remote profiling data matches local profiling for
the same workload so that remote data fidelity is regression-tested.
