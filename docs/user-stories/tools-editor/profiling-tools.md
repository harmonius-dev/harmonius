# User Stories -- 15.5 Profiling Tools

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-15.5.1.1 | engine developer (P-26) |
| US-15.5.1.2 | technical artist (P-13) |
| US-15.5.2.1 | engine developer (P-26) |
| US-15.5.2.2 | technical artist (P-13) |
| US-15.5.3.1 | engine developer (P-26) |
| US-15.5.3.2 | build engineer (P-16)   |
| US-15.5.4.1 | engine developer (P-26) |
| US-15.5.4.2 | build engineer (P-16)   |
| US-15.5.5.1 | engine developer (P-26) |
| US-15.5.5.2 | game designer (P-5)     |
| US-15.5.6.1 | game designer (P-5)     |
| US-15.5.6.2 | technical artist (P-13) |
| US-15.5.7.1 | engine developer (P-26) |
| US-15.5.7.2 | build engineer (P-16)   |
| US-15.5.8.1 | game developer (P-15)   |

1. **US-15.5.1.1** — **As a** engine developer (P-26), **I want** a per-frame CPU timeline showing
   job system tasks and engine ticks as color-coded bars, **so that** I can identify CPU
   bottlenecks.

2. **US-15.5.1.2** — **As a** technical artist (P-13), **I want** flame graph and flat profile views
   with per-subsystem filtering, **so that** I can isolate performance issues without reading code.

3. **US-15.5.2.1** — **As a** engine developer (P-26), **I want** per-pass GPU timestamps aligned
   with the CPU timeline, **so that** I can correlate CPU and GPU work.

4. **US-15.5.2.2** — **As a** technical artist (P-13), **I want** shader occupancy and overdraw
   statistics per render pass, **so that** I can optimize material complexity.

5. **US-15.5.3.1** — **As a** engine developer (P-26), **I want** a memory profiler tracking all
   allocations by subsystem and allocation site with call stacks, **so that** I can find memory
   hotspots.

6. **US-15.5.3.2** — **As a** build engineer (P-16), **I want** live treemap and historical
   allocation timeline views, **so that** I can enforce memory budgets in CI.

7. **US-15.5.4.1** — **As a** engine developer (P-26), **I want** leak detection by comparing
   allocation snapshots at different points in time, **so that** I can catch leaks before release.

8. **US-15.5.4.2** — **As a** build engineer (P-16), **I want** automated leak checks in CI
   comparing pre/post-test snapshots, **so that** regressions are caught automatically.

9. **US-15.5.5.1** — **As a** engine developer (P-26), **I want** a network profiler monitoring
   bandwidth per replication channel with a packet inspector, **so that** I can debug serialization
   issues.

10. **US-15.5.5.2** — **As a** game designer (P-5), **I want** bandwidth graphs with threshold
    alerts, **so that** I can verify my design stays within network budgets.

11. **US-15.5.6.1** — **As a** game designer (P-5), **I want** toggleable HUD stat overlays for FPS,
    frame time, draw calls, and entity count, **so that** I can monitor performance during play
    testing.

12. **US-15.5.6.2** — **As a** technical artist (P-13), **I want** stat overlays recordable to CSV,
    **so that** I can analyze performance trends post-session.

13. **US-15.5.7.1** — **As a** engine developer (P-26), **I want** to connect editor profiling tools
    to a remote game instance over TCP, **so that** I can profile live servers under real player
    load.

14. **US-15.5.7.2** — **As a** build engineer (P-16), **I want** remote profiling to work with
    development kits and mobile devices on the local network, **so that** I can capture
    target-platform data.

15. **US-15.5.8.1** — **As a** game developer (P-15), **I want** per-phase physics profiling metrics
    showing broadphase, narrowphase, and solver timing separately, **so that** I can identify which
    physics stage is the bottleneck in a heavy simulation frame.
