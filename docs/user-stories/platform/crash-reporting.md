# User Stories -- 14.4 Crash Reporting & Diagnostics

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-14.4.1  | player (P-23)           |          |              |
| US-14.4.2  | DevOps engineer (P-16)  |          |              |
| US-14.4.3  | DevOps engineer (P-16)  |          |              |
| US-14.4.4  | game developer (P-15)   |          |              |
| US-14.4.5  | server admin (P-22)     |          |              |
| US-14.4.6  | engine developer (P-26) |          |              |
| US-14.4.7  | engine developer (P-26) |          |              |
| US-14.4.8  | engine developer (P-26) |          |              |
| US-14.4.9  | game developer (P-15)   |          |              |
| US-14.4.10 | engine tester (P-27)    |          |              |
| US-14.4.11 | engine tester (P-27)    |          |              |
| US-14.4.12 | engine tester (P-27)    |          |              |
| US-14.4.13 | engine developer (P-26) |          |              |
| US-14.4.14 | player (P-23)           |          |              |
| US-14.4.15 | DevOps engineer (P-16)  |          |              |
| US-14.4.16 | game developer (P-15)   |          |              |
| US-14.4.17 | engine developer (P-26) |          |              |
| US-14.4.18 | engine tester (P-27)    |          |              |

1. **US-14.4.1** — crashes to automatically generate a dump file and upload it on next launch, so
   that I do not need to do anything to help developers fix the problem
2. **US-14.4.2** — debug symbols uploaded during the build pipeline and crash reports symbolicated
   server-side using build-ID matching, so that every crash report shows function names, file paths,
   and line numbers for immediate debugging
3. **US-14.4.3** — crash aggregation that groups reports by stack signature, tracks frequency, and
   alerts the team on new clusters or spikes, so that we can prioritize fixes by crash volume in the
   live-ops dashboard
4. **US-14.4.4** — structured log records with timestamp, severity, and channel-based filtering
   (rendering, networking, gameplay), so that I can isolate the log output of a specific system
   without noise from others
5. **US-14.4.5** — named performance counters (frame time, draw calls, entity count, network RTT)
   sent to the live-ops backend via telemetry hooks, so that I can monitor performance distributions
   across the entire MMO player base in real time
6. **US-14.4.6** — GPU crash breadcrumbs that write marker values before and after each render pass,
   so that when a GPU hang or device-lost event occurs, the last completed marker identifies the
   faulting pass in the crash report
7. **US-14.4.7** — integration with ETW on Windows, os_signpost on macOS, and perf_event_open on
   Linux, so that engine performance data appears in platform-native profiling tools (Windows
   Performance Analyzer, Instruments, perf)
8. **US-14.4.8** — out-of-process crash capture using platform APIs (MiniDumpWriteDump, Mach
   exception handler, ptrace), so that the crash handler produces a reliable dump even when the
   faulting process has corrupted its own heap
9. **US-14.4.9** — structured logs to also emit to OutputDebugString on Windows, os_log on macOS,
   and sd_journal on Linux, so that I can view engine logs in platform-native tools (debugger
   output, Console.app, journalctl)
10. **US-14.4.10** — automated tests that trigger intentional crashes (segfault, abort, unhandled
    exception) and verify a valid minidump or core dump is produced on each platform, so that crash
    reporting reliability is validated in CI
11. **US-14.4.11** — benchmarks that increment performance counters from all worker threads
    simultaneously and verify zero contention overhead, so that counter accumulation does not
    introduce frame time variance under high thread counts
12. **US-14.4.12** — tests that intentionally trigger GPU hangs and verify the breadcrumb marker
    correctly identifies the injected fault, so that GPU crash diagnostics are validated for each
    graphics API (Vulkan, D3D12, Metal)
13. **US-14.4.13** — crash dump capture to run in a separate out-of-process binary that attaches to
    the faulting process via pipe or socket, so that dump capture succeeds even when the faulting
    process has corrupted its own heap, allocator, or thread state
14. **US-14.4.14** — the engine to retain only a configurable maximum number of crash dumps and
    rotate out the oldest, so that repeated crashes do not consume unbounded disk space on my
    machine
15. **US-14.4.15** — each crash dump to carry key-value metadata (build ID, GPU driver, player ID,
    scene name), so that I can filter and search crash reports on the aggregation server without
    waiting for symbolication
16. **US-14.4.16** — to update the log filter (minimum severity per channel) at runtime from the
    developer console, so that I can increase verbosity for a specific subsystem during a live
    debugging session without restarting the engine
17. **US-14.4.17** — log output destinations implemented as pluggable `LogSink` trait objects
    composed at logger construction time, so that platform-native, file, and remote telemetry sinks
    can be added or replaced without modifying the logger core
18. **US-14.4.18** — tests that corrupt the faulting process's heap before triggering a crash and
    verify the out-of-process handler still produces a valid dump, so that out-of-process capture
    reliability is validated in CI
