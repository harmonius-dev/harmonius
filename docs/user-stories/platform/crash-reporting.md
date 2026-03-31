# User Stories -- 14.4 Crash Reporting & Diagnostics

| ID         | Persona                 |
|------------|-------------------------|
| US-14.4.1  | player (P-23)           |
| US-14.4.2  | player (P-23)           |
| US-14.4.3  | game developer (P-15)   |
| US-14.4.4  | game developer (P-15)   |
| US-14.4.5  | game developer (P-15)   |
| US-14.4.6  | DevOps engineer (P-16)  |
| US-14.4.7  | DevOps engineer (P-16)  |
| US-14.4.8  | DevOps engineer (P-16)  |
| US-14.4.9  | server admin (P-22)     |
| US-14.4.10 | engine developer (P-26) |
| US-14.4.11 | engine developer (P-26) |
| US-14.4.12 | engine developer (P-26) |
| US-14.4.13 | engine developer (P-26) |
| US-14.4.14 | engine tester (P-27)    |
| US-14.4.15 | engine tester (P-27)    |
| US-14.4.16 | engine tester (P-27)    |
| US-14.4.17 | engine tester (P-27)    |

## Crash Handling

1. **US-14.4.1** — **As a** player (P-23), **I want** crashes to automatically generate a dump file
   and upload it on next launch, **so that** I do not need to do anything to help developers fix the
   problem.
2. **US-14.4.2** — **As a** player (P-23), **I want** the engine to retain only a configurable
   maximum number of crash dumps and rotate out the oldest, **so that** repeated crashes do not
   consume unbounded disk space on my machine.

## Developer Diagnostics

3. **US-14.4.3** — **As a** game developer (P-15), **I want** structured log records with timestamp,
   severity, and channel-based filtering, **so that** I can isolate the log output of a specific
   subsystem without noise from others.
4. **US-14.4.4** — **As a** game developer (P-15), **I want** to update the log filter at runtime
   from the developer console, **so that** I can increase verbosity for a specific subsystem during
   a live debugging session without restarting.
5. **US-14.4.5** — **As a** game developer (P-15), **I want** structured logs to emit to
   OutputDebugString on Windows, os_log on macOS, and sd_journal on Linux, **so that** I can view
   engine logs in platform-native tools.

## DevOps and Build Pipeline

6. **US-14.4.6** — **As a** DevOps engineer (P-16), **I want** debug symbols uploaded during the
   build pipeline and crash reports symbolicated server-side, **so that** every report shows
   function names, file paths, and line numbers.
7. **US-14.4.7** — **As a** DevOps engineer (P-16), **I want** crash aggregation that groups reports
   by stack signature, tracks frequency, and alerts on new clusters or spikes, **so that** we
   prioritize fixes by crash volume.
8. **US-14.4.8** — **As a** DevOps engineer (P-16), **I want** each crash dump to carry key-value
   metadata (build ID, GPU driver, player ID, scene name), **so that** I can filter and search crash
   reports without waiting for symbolication.

## Server Administration

9. **US-14.4.9** — **As a** server admin (P-22), **I want** named performance counters (frame time,
   draw calls, network RTT) sent to the live-ops backend via telemetry hooks, **so that** I can
   monitor performance across the player base in real time.

## Engine Developer -- Internals

10. **US-14.4.10** — **As an** engine developer (P-26), **I want** GPU crash breadcrumbs that write
    marker values before and after each render pass, **so that** when a GPU hang occurs the last
    completed marker identifies the faulting pass.
11. **US-14.4.11** — **As an** engine developer (P-26), **I want** out-of-process crash capture
    using platform APIs, **so that** the crash handler produces a reliable dump even when the
    faulting process has corrupted its own heap.
12. **US-14.4.12** — **As an** engine developer (P-26), **I want** integration with ETW on Windows,
    os_signpost on macOS, and perf_event_open on Linux, **so that** engine performance data appears
    in platform-native profiling tools.
13. **US-14.4.13** — **As an** engine developer (P-26), **I want** log output destinations
    implemented as pluggable `LogSink` trait objects, **so that** platform-native, file, and remote
    telemetry sinks can be added without modifying the logger core.

## Engine Tester -- Validation

14. **US-14.4.14** — **As an** engine tester (P-27), **I want** tests that trigger intentional
    crashes and verify a valid dump is produced on each platform, **so that** crash reporting
    reliability is validated in CI.
15. **US-14.4.15** — **As an** engine tester (P-27), **I want** benchmarks that increment
    performance counters from all worker threads and verify zero contention overhead, **so that**
    counter accumulation does not introduce frame time variance.
16. **US-14.4.16** — **As an** engine tester (P-27), **I want** tests that trigger GPU hangs and
    verify the breadcrumb marker identifies the injected fault, **so that** GPU crash diagnostics
    are validated for each graphics API.
17. **US-14.4.17** — **As an** engine tester (P-27), **I want** tests that corrupt the faulting
    process's heap before triggering a crash and verify the out-of-process handler still produces a
    valid dump, **so that** out-of-process capture reliability is validated in CI.
