# 15.5 — Profiling Tools

## CPU Profiling

| ID | Feature |
| ---------- | ------------------------------- |
| F-15.5.1 | Frame Profiler (CPU Timeline) |

1. **F-15.5.1** — Captures and displays a per-frame CPU timeline showing all job system tasks,
   engine subsystem ticks, and user script execution as color-coded bars on a swimlane chart.
   Supports flame graph and flat profile views, filtering by thread or subsystem, and frame-to-frame
   comparison. The profiler must handle engines running at hundreds of FPS without introducing
   measurement overhead exceeding 1%. macOS, integrates with Instruments via os_signpost. On Linux,
   supports perf integration for hardware counters.
   - **Deps:** F-7.2.1
   - **Platform:** On Windows, integrates with ETW for kernel-level thread scheduling data. On

## GPU Profiling

| ID | Feature |
| ---------- | ------------------------------------------ |
| F-15.5.2 | GPU Profiler (Pass Timing and Occupancy) |

1. **F-15.5.2** — Captures GPU timestamps per render graph pass and presents them on a timeline
   aligned with the CPU frame profiler. Reports per-pass duration, shader occupancy, wave
   utilization, and overdraw statistics. Supports vendor-specific counters on AMD (RGPMT), NVIDIA
   (Nsight), and Apple (Vulkan trace) GPUs. NVIDIA Ada exposes SM throughput; Apple M-series
   exposes shader ALU utilization.
   - **Deps:** F-2.1.1, F-15.5.1
   - **Platform:** Vendor-specific counter availability varies. AMD RDNA exposes CU occupancy;

## Memory Profiling

| ID | Feature |
| ---------- | --------------------------------------- |
| F-15.5.3 | Memory Profiler (Allocation Tracking) |
| F-15.5.4 | Leak Detection |

1. **F-15.5.3** — Tracks all CPU and GPU memory allocations by subsystem, asset type, and allocation
   site. Displays a live treemap of memory consumption, historical allocation timeline, and
   per-frame allocation rate. Provides call-stack capture for each allocation to locate memory
   hotspots. Critical for enforcing memory budgets on MMO clients that must run for extended
   sessions. backtrace on macOS/Linux.
   - **Deps:** F-7.7.1
   - **Platform:** Stack capture uses platform-specific APIs: CaptureStackBackTrace on Windows,
2. **F-15.5.4** — Detects memory leaks by comparing allocation snapshots taken at different points
   in time. Reports allocations present in the later snapshot but absent in the earlier one, grouped
   by allocation site and asset type. Supports automated leak checks in CI by comparing snapshots
   before and after test scenarios. (F-15.5.7) to profile mobile and console targets.
   - **Deps:** F-15.5.3
   - **Platform:** Desktop only. Not available on mobile or console runtime. Use remote profiling

## Network Profiling

| ID | Feature |
| ---------- | --------------------------------------------------- |
| F-15.5.5 | Network Profiler (Bandwidth and Packet Inspector) |

1. **F-15.5.5** — Monitors network bandwidth per replication channel, RPC category, and entity
   relevance set. Provides a packet inspector that decodes individual packets into structured fields
   for debugging serialization issues. Graphs upstream and downstream bandwidth over time and flags
   spikes that exceed budget thresholds. Essential for tuning MMO replication in crowded zones.
   - **Deps:** F-12.1.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Overlays and Remote Profiling

| ID | Feature |
| ---------- | ------------------ |
| F-15.5.6 | Stat Overlays |
| F-15.5.7 | Remote Profiling |

1. **F-15.5.6** — Renders configurable HUD overlays on top of the game viewport showing real-time
   statistics: FPS, frame time, draw call count, triangle count, GPU memory, CPU thread utilization,
   network bandwidth, and entity count. Overlays are toggleable individually and can be recorded to
   CSV for post-session analysis. compact layout to avoid obscuring the small screen.
   - **Deps:** F-15.5.1, F-15.5.2
   - **Platform:** Stat overlays render on all platforms including mobile. Mobile overlays use a
2. **F-15.5.7** — Connects the editor-side profiling tools to a live game server or client over TCP
   for remote data collection. All profiler views (CPU timeline, GPU passes, memory, network)
   function identically when driven by remote data. Supports connecting to dedicated servers,
   development kits, and mobile devices on the local network. Vital for profiling MMO servers under
   real player load. capture granularity to limit bandwidth to under 10 Mbps.
   - **Deps:** F-15.5.1, F-15.5.2, F-15.5.3, F-15.5.5
   - **Platform:** Requires a low-overhead binary protocol; data is streamed with configurable

## Subsystem Profilers

| ID | Feature |
| ---------- | ----------------------------------------------------- |
| F-15.5.8 | Per-Subsystem Profiler Trackers |
| F-15.5.9 | Side-by-Side Frame Comparison |
| F-15.5.10 | Logic Graph-Defined Profiler Zones |

1. **F-15.5.8** — Dedicated profiler trackers for all major engine subsystems: ECS system timing
   (per-system name), physics (broadphase/narrowphase/solver), audio (callback duration, voice
   count, underruns), asset streaming (queue depth, cache hit rate, I/O bandwidth), VFX (particle
   count, GPU dispatch time), UI (layout/paint pass duration, widget count), job system (worker
   busy/idle ratio, steal count, queue depth), and arena memory (per-thread watermarks, reset
   counts). Each tracker integrates into the CPU timeline and memory profiler views.
   - **Deps:** F-15.5.1, F-15.5.3
   - **Platform:** All platforms. Remote profiling streams subsystem data.
2. **F-15.5.9** — Side-by-side frame comparison in the CPU timeline. Select two captured frames and
   view them aligned vertically with matching zone names. Zones with timing differences exceeding a
   configurable threshold are highlighted. Enables quick identification of performance regressions
   between frames, builds, or code changes.
   - **Deps:** F-15.5.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
3. **F-15.5.10** — User-defined profiler zones created from logic graph nodes. A dedicated "Profiler
   Zone" node wraps a subgraph region and registers a named zone at codegen time. Zone names are
   stable across hot-reloads. User zones appear alongside engine zones in the CPU timeline with a
   distinct color category.
   - **Deps:** F-15.5.1, F-15.8.4 (Gameplay Logic Graphs)
   - **Platform:** All platforms. Zones are compiled into native code.
