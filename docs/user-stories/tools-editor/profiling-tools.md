# User Stories: Profiling Tools

## F-15.5.1 Frame Profiler (CPU Timeline)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.5.1.1 | developer (P-15)        |          |              |
| US-15.5.1.2 | tech artist (P-13)      |          |              |
| US-15.5.1.3 | engine developer (P-26) |          |              |
| US-15.5.1.4 | engine tester (P-27)    |          |              |

1. **US-15.5.1.1** — a per-frame CPU timeline showing all job system tasks and subsystem ticks as
   color-coded bars on a swimlane chart
   - **Acceptance:** I can identify which system is causing frame time spikes without instrumenting
     code manually
2. **US-15.5.1.2** — to filter the CPU timeline by thread or subsystem and switch between flame
   graph and flat profile views
   - **Acceptance:** I can isolate rendering-related CPU work from gameplay and physics
3. **US-15.5.1.3** — frame-to-frame comparison in the CPU profiler
   - **Acceptance:** I can identify regressions by comparing a slow frame against a baseline frame
4. **US-15.5.1.4** — to verify that profiler instrumentation adds less than 1% measurement overhead
   - **Acceptance:** profiling data accurately reflects production performance

## F-15.5.2 GPU Profiler (Pass Timing and Occupancy)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.5.2.1 | tech artist (P-13)      |          |              |
| US-15.5.2.2 | developer (P-15)        |          |              |
| US-15.5.2.3 | engine developer (P-26) |          |              |
| US-15.5.2.4 | engine tester (P-27)    |          |              |

1. **US-15.5.2.1** — GPU timestamps per render graph pass presented on a timeline aligned with the
   CPU frame profiler
   - **Acceptance:** I can see which render passes are most expensive and where GPU time is spent
2. **US-15.5.2.2** — per-pass shader occupancy, wave utilization, and overdraw statistics
   - **Acceptance:** I can optimize shader workloads and reduce GPU bottlenecks
3. **US-15.5.2.3** — vendor-specific GPU counters (AMD RGPMT, NVIDIA Nsight, Apple Metal System
   Trace)
   - **Acceptance:** I can access hardware-level performance data for deep optimization on each GPU
     architecture
4. **US-15.5.2.4** — to verify that GPU profiling works consistently across AMD, NVIDIA, and Apple
   GPUs with appropriate vendor counter availability
   - **Acceptance:** performance analysis is reliable on all target hardware

## F-15.5.3 Memory Profiler (Allocation Tracking)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.5.3.1 | developer (P-15)        |          |              |
| US-15.5.3.2 | tech artist (P-13)      |          |              |
| US-15.5.3.3 | engine developer (P-26) |          |              |
| US-15.5.3.4 | DevOps engineer (P-16)  |          |              |

1. **US-15.5.3.1** — all CPU and GPU memory allocations tracked by subsystem, asset type, and
   allocation site with call-stack capture
   - **Acceptance:** I can locate memory hotspots and enforce budgets
2. **US-15.5.3.2** — a live treemap of memory consumption and a historical allocation timeline
   - **Acceptance:** I can visualize which asset types consume the most memory and identify growth
     trends
3. **US-15.5.3.3** — per-frame allocation rate tracking
   - **Acceptance:** I can detect allocation spikes that may cause garbage collection stalls or
     fragmentation
4. **US-15.5.3.4** — memory profiling data available in CI reports
   - **Acceptance:** automated tests can fail when a subsystem exceeds its allocated memory budget

## F-15.5.4 Leak Detection

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.5.4.1 | developer (P-15)        |          |              |
| US-15.5.4.2 | DevOps engineer (P-16)  |          |              |
| US-15.5.4.3 | engine developer (P-26) |          |              |
| US-15.5.4.4 | engine tester (P-27)    |          |              |

1. **US-15.5.4.1** — to compare memory allocation snapshots taken at different points in time to
   detect leaks
   - **Acceptance:** I can identify allocations that grow unboundedly during extended play sessions
2. **US-15.5.4.2** — automated leak detection in CI by comparing snapshots before and after test
   scenarios
   - **Acceptance:** memory leaks are caught before they reach production builds
3. **US-15.5.4.3** — leak reports grouped by allocation site and asset type
   - **Acceptance:** I can prioritize fixing the highest-impact leak sources first
4. **US-15.5.4.4** — to run extended play sessions with leak detection enabled and verify zero net
   allocation growth
   - **Acceptance:** the engine is stable for long-running game sessions

## F-15.5.5 Network Profiler (Bandwidth and Packet Inspector)

| ID          | Persona                | Features | Requirements |
|-------------|------------------------|----------|--------------|
| US-15.5.5.1 | developer (P-15)       |          |              |
| US-15.5.5.2 | server admin (P-22)    |          |              |
| US-15.5.5.3 | DevOps engineer (P-16) |          |              |
| US-15.5.5.4 | engine tester (P-27)   |          |              |

1. **US-15.5.5.1** — to monitor network bandwidth per replication channel, RPC category, and entity
   relevance set
   - **Acceptance:** I can identify which systems consume the most bandwidth and optimize
     accordingly
2. **US-15.5.5.2** — a packet inspector that decodes individual packets into structured fields
   - **Acceptance:** I can debug serialization issues and verify protocol correctness
3. **US-15.5.5.3** — bandwidth graphs over time that flag spikes exceeding budget thresholds
   - **Acceptance:** I can detect and investigate network usage anomalies before they affect players
4. **US-15.5.5.4** — to profile network bandwidth in simulated crowded zones
   - **Acceptance:** I can verify that replication stays within budget under worst-case player
     density

## F-15.5.6 Stat Overlays

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.5.6.1 | designer (P-5)       |          |              |
| US-15.5.6.2 | tech artist (P-13)   |          |              |
| US-15.5.6.3 | developer (P-15)     |          |              |
| US-15.5.6.4 | engine tester (P-27) |          |              |

1. **US-15.5.6.1** — configurable HUD overlays showing FPS, frame time, draw calls, and triangle
   count on the viewport
   - **Acceptance:** I can monitor performance impact while editing levels
2. **US-15.5.6.2** — to record stat overlay data to CSV for post-session analysis
   - **Acceptance:** I can track performance trends across editing sessions and share data with the
     optimization team
3. **US-15.5.6.3** — to toggle individual stat overlays (GPU memory, CPU thread utilization, network
   bandwidth, entity count) independently
   - **Acceptance:** I can focus on the specific metrics relevant to my current investigation
4. **US-15.5.6.4** — stat overlays to render on all platforms including mobile with a compact layout
   - **Acceptance:** on-device performance monitoring does not obscure the small screen

## F-15.5.7 Remote Profiling

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.5.7.1 | developer (P-15)        |          |              |
| US-15.5.7.2 | server admin (P-22)     |          |              |
| US-15.5.7.3 | DevOps engineer (P-16)  |          |              |
| US-15.5.7.4 | engine developer (P-26) |          |              |

1. **US-15.5.7.1** — to connect the editor profiler to a live game server over TCP
   - **Acceptance:** I can diagnose performance issues under real player load without deploying
     debug builds
2. **US-15.5.7.2** — remote profiling to connect to dedicated servers and development kits on the
   local network
   - **Acceptance:** I can monitor server performance in real-time during load testing
3. **US-15.5.7.3** — remote profiling to connect to mobile devices on the local network
   - **Acceptance:** I can capture CPU, GPU, and memory data from target hardware without on-device
     tools
4. **US-15.5.7.4** — configurable capture granularity that limits profiling bandwidth to under 10
   Mbps
   - **Acceptance:** remote profiling does not perturb the system under measurement
