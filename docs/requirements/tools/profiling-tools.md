# R-15.5 -- Profiling Tools Requirements

## Requirements

1. **R-15.5.1** — The engine **SHALL** capture a per-frame CPU timeline with flame graph, flat
   profile, and swimlane views, introducing less than 1% measurement overhead.
   - **Rationale:** Low-overhead profiling enables always-on capture during development.
   - **Verification:** Run a benchmark scene, verify profiler overhead is under 1% by comparing
     frame times with profiler enabled and disabled.

2. **R-15.5.2** — The engine **SHALL** capture per-pass GPU timestamps aligned with the CPU timeline
   and report shader occupancy and overdraw statistics.
   - **Rationale:** GPU profiling enables identifying render pass bottlenecks and shader
     inefficiencies.
   - **Verification:** Render a scene, verify GPU pass timings appear in the timeline and sum to the
     total GPU frame time.

3. **R-15.5.3** — The engine **SHALL** track all CPU and GPU memory allocations by subsystem and
   allocation site with call-stack capture, live treemap, and historical timeline.
   - **Rationale:** Memory tracking is critical for enforcing budgets on long-running clients.
   - **Verification:** Allocate a known-size block, verify it appears in the treemap at the correct
     size and call site.

4. **R-15.5.4** — The engine **SHALL** detect memory leaks by comparing allocation snapshots and
   support automated leak checks in CI.
   - **Rationale:** Automated leak detection prevents regressions from reaching release builds.
   - **Verification:** Intentionally leak an allocation between two snapshots and verify the leak
     detector reports it.

5. **R-15.5.5** — The engine **SHALL** provide a network profiler monitoring bandwidth per
   replication channel and entity set, with a packet inspector for structured field decoding.
   - **Rationale:** Network profiling is essential for tuning replication in multiplayer scenarios.
   - **Verification:** Send a known-size RPC, verify the network profiler reports the correct byte
     count and decodes fields.

6. **R-15.5.6** — The engine **SHALL** render toggleable stat overlays on all platforms showing FPS,
   frame time, draw calls, triangle count, GPU memory, and entity count, with CSV recording.
   - **Rationale:** Real-time overlays enable performance monitoring during play testing on any
     device.
   - **Verification:** Enable the FPS overlay, record 60 frames to CSV, and verify the CSV contains
     60 rows of valid data.

7. **R-15.5.7** — The engine **SHALL** support remote profiling over TCP connecting editor tools to
   live game instances, servers, development kits, and mobile devices.
   - **Rationale:** Target-platform profiling requires remote data collection without running the
     editor on the target.
   - **Verification:** Connect to a remote instance, capture a CPU timeline, and verify the data
     matches a local capture.

8. **R-15.5.8** — The engine **SHALL** track per-ECS-system execution time with the system name,
   showing system bottlenecks in the CPU timeline.
   - **Rationale:** ECS system profiling is critical for identifying gameplay logic bottlenecks.
   - **Verification:** Run a scene with 20 systems, verify each system appears with its name and
     timing in the CPU timeline.

9. **R-15.5.9** — The engine **SHALL** track per-thread arena watermarks and reset counts, reporting
   excessive arena usage.
   - **Rationale:** Arena allocators can waste memory if watermarks grow unbounded.
   - **Verification:** Trigger a large allocation in a per-thread arena and verify the profiler
     reports the watermark increase.

10. **R-15.5.10** — The engine **SHALL** report GPU VRAM usage broken down by resource type
    (textures, buffers, render targets) with a live treemap visualization.
    - **Rationale:** VRAM breakdown identifies which resource types consume the most memory for
      targeted optimization.
    - **Verification:** Load a scene with known texture and buffer sizes and verify the VRAM treemap
      matches expected allocations.

11. **R-15.5.11** — The engine **SHALL** profile physics subsystem timing (broadphase, narrowphase,
    solver) and display per-step metrics.
    - **Rationale:** Physics is often a frame-time bottleneck; per-phase timing reveals which phase
      to optimize.
    - **Verification:** Run a physics-heavy scene and verify broadphase, narrowphase, and solver
      timings appear as separate entries.

12. **R-15.5.12** — The engine **SHALL** profile audio subsystem timing (callback duration, active
    voice count, buffer underruns).
    - **Rationale:** Audio callback overruns cause audible glitches that are hard to diagnose
      without dedicated profiling.
    - **Verification:** Play 64 simultaneous voices, verify the profiler shows callback duration and
      voice count.

13. **R-15.5.13** — The engine **SHALL** profile asset streaming performance (load queue depth,
    cache hit rate, I/O bandwidth).
    - **Rationale:** Streaming bottlenecks cause visible pop-in and hitches during traversal.
    - **Verification:** Stream assets during camera movement and verify the profiler shows queue
      depth, hit rate, and bandwidth.

14. **R-15.5.14** — The engine **SHALL** profile VFX subsystem metrics (active particle count, GPU
    compute dispatch time).
    - **Rationale:** VFX can silently consume GPU budget; dedicated metrics enable targeted particle
      budget enforcement.
    - **Verification:** Spawn a particle system with 10,000 particles and verify the profiler shows
      the correct count and dispatch time.

15. **R-15.5.15** — The engine **SHALL** profile UI subsystem timing (layout pass duration, paint
    pass duration, widget count).
    - **Rationale:** Complex UI hierarchies can become frame-time bottlenecks without dedicated
      profiling.
    - **Verification:** Open a panel with 500 widgets and verify layout and paint pass durations
      appear in the profiler.

16. **R-15.5.16** — The engine **SHALL** support side-by-side frame comparison in the CPU timeline
    to identify performance regressions.
    - **Rationale:** Comparing frames reveals timing regressions that absolute numbers miss.
    - **Verification:** Capture two frames, enable comparison mode, and verify differing zones are
      highlighted.

17. **R-15.5.17** — The engine **SHALL** profile job system utilization (worker busy/idle ratio,
    work-steal count, queue depth).
    - **Rationale:** Job system profiling reveals load imbalance and scheduling inefficiency.
    - **Verification:** Run a parallel workload across 8 workers and verify per-worker busy/idle
      ratios and steal counts are displayed.

18. **R-15.5.18** — The engine **SHALL** profile network latency (RTT histogram, jitter, packet loss
    rate, retransmit count).
    - **Rationale:** Latency profiling complements bandwidth profiling for multiplayer optimization.
    - **Verification:** Introduce artificial latency and verify the RTT histogram and jitter metrics
      update correctly.

19. **R-15.5.19** — The engine **SHALL** support user-defined profiler zones from logic graphs, with
    zone names registered at codegen time and stable across hot-reloads.
    - **Rationale:** User-defined zones enable designers to profile their own logic without engine
      modifications.
    - **Verification:** Add a profiler zone node in a logic graph, compile, and verify the zone
      appears in the CPU timeline with the correct name.
