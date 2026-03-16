# 15.5 — Profiling Tools

## CPU Profiling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.5.1 | Frame Profiler (CPU Timeline) | Captures and displays a per-frame CPU timeline showing all job system tasks, engine subsystem ticks, and user script execution as color-coded bars on a swimlane chart. Supports flame graph and flat profile views, filtering by thread or subsystem, and frame-to-frame comparison. The profiler must handle engines running at hundreds of FPS without introducing measurement overhead exceeding 1%. macOS, integrates with Instruments via os_signpost. On Linux, supports perf integration for hardware counters. | R-15.5.1 | F-7.2.1 | On Windows, integrates with ETW for kernel-level thread scheduling data. On |

## GPU Profiling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.5.2 | GPU Profiler (Pass Timing and Occupancy) | Captures GPU timestamps per render graph pass and presents them on a timeline aligned with the CPU frame profiler. Reports per-pass duration, shader occupancy, wave utilization, and overdraw statistics. Supports vendor-specific counters on AMD (RGPMT), NVIDIA (Nsight), and Apple (Metal System Trace) GPUs. NVIDIA Ada exposes SM throughput; Apple M-series exposes shader ALU utilization. | R-15.5.2 | F-2.1.1, F-15.5.1 | Vendor-specific counter availability varies. AMD RDNA exposes CU occupancy; |

## Memory Profiling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.5.3 | Memory Profiler (Allocation Tracking) | Tracks all CPU and GPU memory allocations by subsystem, asset type, and allocation site. Displays a live treemap of memory consumption, historical allocation timeline, and per-frame allocation rate. Provides call-stack capture for each allocation to locate memory hotspots. Critical for enforcing memory budgets on MMO clients that must run for extended sessions. backtrace on macOS/Linux. | R-15.5.3 | F-7.7.1 | Stack capture uses platform-specific APIs: CaptureStackBackTrace on Windows, |
| F-15.5.4 | Leak Detection | Detects memory leaks by comparing allocation snapshots taken at different points in time. Reports allocations present in the later snapshot but absent in the earlier one, grouped by allocation site and asset type. Supports automated leak checks in CI by comparing snapshots before and after test scenarios. (F-15.5.7) to profile mobile and console targets. | R-15.5.4 | F-15.5.3 | Desktop only. Not available on mobile or console runtime. Use remote profiling |

## Network Profiling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.5.5 | Network Profiler (Bandwidth and Packet Inspector) | Monitors network bandwidth per replication channel, RPC category, and entity relevance set. Provides a packet inspector that decodes individual packets into structured fields for debugging serialization issues. Graphs upstream and downstream bandwidth over time and flags spikes that exceed budget thresholds. Essential for tuning MMO replication in crowded zones. | R-15.5.5 | F-12.1.1 | Desktop only. Not available on mobile or console runtime. |

## Overlays and Remote Profiling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.5.6 | Stat Overlays | Renders configurable HUD overlays on top of the game viewport showing real-time statistics: FPS, frame time, draw call count, triangle count, GPU memory, CPU thread utilization, network bandwidth, and entity count. Overlays are toggleable individually and can be recorded to CSV for post-session analysis. compact layout to avoid obscuring the small screen. | R-15.5.6 | F-15.5.1, F-15.5.2 | Stat overlays render on all platforms including mobile. Mobile overlays use a |
| F-15.5.7 | Remote Profiling | Connects the editor-side profiling tools to a live game server or client over TCP for remote data collection. All profiler views (CPU timeline, GPU passes, memory, network) function identically when driven by remote data. Supports connecting to dedicated servers, development kits, and mobile devices on the local network. Vital for profiling MMO servers under real player load. capture granularity to limit bandwidth to under 10 Mbps. | R-15.5.7 | F-15.5.1, F-15.5.2, F-15.5.3, F-15.5.5 | Requires a low-overhead binary protocol; data is streamed with configurable |
