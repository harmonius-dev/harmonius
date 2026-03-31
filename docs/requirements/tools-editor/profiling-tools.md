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
