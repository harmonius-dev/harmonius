# User Stories: Profiling Tools

## F-15.5.1 Frame Profiler (CPU Timeline)

## US-15.5.1.1 Developer Identifies CPU Bottleneck
**As a** developer (P-15), **I want** a per-frame CPU timeline showing all job system tasks and
subsystem ticks as color-coded bars on a swimlane chart, **so that** I can identify which system is
causing frame time spikes without instrumenting code manually.

## US-15.5.1.2 Tech Artist Filters by Subsystem
**As a** tech artist (P-13), **I want** to filter the CPU timeline by thread or subsystem and switch
between flame graph and flat profile views, **so that** I can isolate rendering-related CPU work
from gameplay and physics.

## US-15.5.1.3 Engine Dev Compares Frames
**As an** engine developer (P-26), **I want** frame-to-frame comparison in the CPU profiler, **so
that** I can identify regressions by comparing a slow frame against a baseline frame.

## US-15.5.1.4 Engine Tester Validates Measurement Overhead
**As an** engine tester (P-27), **I want** to verify that profiler instrumentation adds less than 1%
measurement overhead, **so that** profiling data accurately reflects production performance.

## F-15.5.2 GPU Profiler (Pass Timing and Occupancy)

## US-15.5.2.1 Tech Artist Views Pass Timings
**As a** tech artist (P-13), **I want** GPU timestamps per render graph pass presented on a timeline
aligned with the CPU frame profiler, **so that** I can see which render passes are most expensive
and where GPU time is spent.

## US-15.5.2.2 Developer Reads Occupancy Metrics
**As a** developer (P-15), **I want** per-pass shader occupancy, wave utilization, and overdraw
statistics, **so that** I can optimize shader workloads and reduce GPU bottlenecks.

## US-15.5.2.3 Engine Dev Uses Vendor Counters
**As an** engine developer (P-26), **I want** vendor-specific GPU counters (AMD RGPMT, NVIDIA
Nsight, Apple Metal System Trace), **so that** I can access hardware-level performance data for deep
optimization on each GPU architecture.

## US-15.5.2.4 Engine Tester Validates Cross-Platform Profiling
**As an** engine tester (P-27), **I want** to verify that GPU profiling works consistently across
AMD, NVIDIA, and Apple GPUs with appropriate vendor counter availability, **so that** performance
analysis is reliable on all target hardware.

## F-15.5.3 Memory Profiler (Allocation Tracking)

## US-15.5.3.1 Developer Tracks Allocations by Subsystem
**As a** developer (P-15), **I want** all CPU and GPU memory allocations tracked by subsystem, asset
type, and allocation site with call-stack capture, **so that** I can locate memory hotspots and
enforce budgets.

## US-15.5.3.2 Tech Artist Views Memory Treemap
**As a** tech artist (P-13), **I want** a live treemap of memory consumption and a historical
allocation timeline, **so that** I can visualize which asset types consume the most memory and
identify growth trends.

## US-15.5.3.3 Engine Dev Monitors Per-Frame Allocation Rate
**As an** engine developer (P-26), **I want** per-frame allocation rate tracking, **so that** I can
detect allocation spikes that may cause garbage collection stalls or fragmentation.

## US-15.5.3.4 DevOps Enforces Memory Budgets
**As a** DevOps engineer (P-16), **I want** memory profiling data available in CI reports, **so
that** automated tests can fail when a subsystem exceeds its allocated memory budget.

## F-15.5.4 Leak Detection

## US-15.5.4.1 Developer Detects Leaks by Snapshot Comparison
**As a** developer (P-15), **I want** to compare memory allocation snapshots taken at different
points in time to detect leaks, **so that** I can identify allocations that grow unboundedly during
extended play sessions.

## US-15.5.4.2 DevOps Automates Leak Checks in CI
**As a** DevOps engineer (P-16), **I want** automated leak detection in CI by comparing snapshots
before and after test scenarios, **so that** memory leaks are caught before they reach production
builds.

## US-15.5.4.3 Engine Dev Groups Leaks by Site
**As an** engine developer (P-26), **I want** leak reports grouped by allocation site and asset
type, **so that** I can prioritize fixing the highest-impact leak sources first.

## US-15.5.4.4 Engine Tester Validates Long-Session Stability
**As an** engine tester (P-27), **I want** to run extended play sessions with leak detection enabled
and verify zero net allocation growth, **so that** the engine is stable for long-running game
sessions.

## F-15.5.5 Network Profiler (Bandwidth and Packet Inspector)

## US-15.5.5.1 Developer Monitors Bandwidth per Channel
**As a** developer (P-15), **I want** to monitor network bandwidth per replication channel, RPC
category, and entity relevance set, **so that** I can identify which systems consume the most
bandwidth and optimize accordingly.

## US-15.5.5.2 Server Admin Inspects Packets
**As a** server admin (P-22), **I want** a packet inspector that decodes individual packets into
structured fields, **so that** I can debug serialization issues and verify protocol correctness.

## US-15.5.5.3 DevOps Detects Bandwidth Spikes
**As a** DevOps engineer (P-16), **I want** bandwidth graphs over time that flag spikes exceeding
budget thresholds, **so that** I can detect and investigate network usage anomalies before they
affect players.

## US-15.5.5.4 Engine Tester Validates Crowded Zone Performance
**As an** engine tester (P-27), **I want** to profile network bandwidth in simulated crowded zones,
**so that** I can verify that replication stays within budget under worst-case player density.

## F-15.5.6 Stat Overlays

## US-15.5.6.1 Designer Monitors FPS in Viewport
**As a** designer (P-5), **I want** configurable HUD overlays showing FPS, frame time, draw calls,
and triangle count on the viewport, **so that** I can monitor performance impact while editing
levels.

## US-15.5.6.2 Tech Artist Records Stats to CSV
**As a** tech artist (P-13), **I want** to record stat overlay data to CSV for post-session
analysis, **so that** I can track performance trends across editing sessions and share data with the
optimization team.

## US-15.5.6.3 Developer Toggles Individual Overlays
**As a** developer (P-15), **I want** to toggle individual stat overlays (GPU memory, CPU thread
utilization, network bandwidth, entity count) independently, **so that** I can focus on the specific
metrics relevant to my current investigation.

## US-15.5.6.4 Engine Tester Validates Mobile Overlay Layout
**As an** engine tester (P-27), **I want** stat overlays to render on all platforms including mobile
with a compact layout, **so that** on-device performance monitoring does not obscure the small
screen.

## F-15.5.7 Remote Profiling

## US-15.5.7.1 Developer Profiles Live Server
**As a** developer (P-15), **I want** to connect the editor profiler to a live game server over TCP,
**so that** I can diagnose performance issues under real player load without deploying debug builds.

## US-15.5.7.2 Server Admin Profiles Dedicated Server
**As a** server admin (P-22), **I want** remote profiling to connect to dedicated servers and
development kits on the local network, **so that** I can monitor server performance in real-time
during load testing.

## US-15.5.7.3 DevOps Profiles Mobile Devices
**As a** DevOps engineer (P-16), **I want** remote profiling to connect to mobile devices on the
local network, **so that** I can capture CPU, GPU, and memory data from target hardware without
on-device tools.

## US-15.5.7.4 Engine Dev Controls Capture Bandwidth
**As an** engine developer (P-26), **I want** configurable capture granularity that limits profiling
bandwidth to under 10 Mbps, **so that** remote profiling does not perturb the system under
measurement.
