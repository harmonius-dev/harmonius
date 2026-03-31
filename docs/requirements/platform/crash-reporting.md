# R-14.4 — Crash Reporting & Diagnostics Requirements

## Crash Handling

1. **R-14.4.1** — The engine **SHALL** install a process-wide crash handler that intercepts
   unhandled exceptions, segfaults, and aborts, then writes a platform-native crash dump (minidump
   on Windows, core dump on macOS/Linux) containing the faulting thread's stack, register state, and
   key memory regions, without requiring any user interaction.
   - **Rationale:** Every crash must produce an actionable artifact automatically so players never
     need to perform manual steps and developers receive complete diagnostic data.
   - **Verification:** Integration test per platform: trigger a null-pointer dereference in a child
     process; verify a valid dump is written. Load the dump in a platform debugger and verify stack
     and register state. Verify out-of-process capture works with a corrupted heap.

2. **R-14.4.2** — The engine **SHALL** upload debug symbols (PDB on Windows, dSYM on macOS, DWARF on
   Linux) to a crash aggregation service during the build pipeline, indexed by build ID, enabling
   server-side symbolication into function names, file paths, and line numbers.
   - **Rationale:** Server-side symbolication decouples crash analysis from local debug symbol
     availability, allowing any team member to diagnose crashes from any build.
   - **Verification:** CI test: build a release binary, upload symbols, trigger a crash, verify the
     aggregation service symbolicates with correct function names and line numbers.

3. **R-14.4.3** — The engine **SHALL** group crash reports by symbolicated stack signature, track
   frequency over time, and alert when a new cluster appears or an existing cluster's frequency
   exceeds a configurable threshold.
   - **Rationale:** Clustering and alerting enables rapid triage by surfacing the highest-impact
     crashes first and detecting regression spikes.
   - **Verification:** Integration test: submit 50 reports with 3 distinct signatures; verify 3
     clusters. Inject a spike of 20 reports within 5 minutes for one cluster; verify alert fires.

## Diagnostics

4. **R-14.4.4** — The engine **SHALL** emit structured log records (timestamp, severity, channel,
   key-value fields) written to an async ring buffer and flushed to disk and/or remote telemetry
   without blocking the calling thread.
   - **Rationale:** Structured logging with channel filtering isolates subsystem output. Async
     flushing prevents logging from impacting frame time.
   - **Verification:** Unit test: emit 10,000 records across 4 channels; verify all appear with
     correct fields. Verify channel/severity filtering. Assert log emission takes under 1 us per
     record.

5. **R-14.4.5** — The engine **SHALL** expose named performance counters as lock-free per-thread
   accumulators merged at frame boundaries, readable by the profiler, HUD, and telemetry systems.
   - **Rationale:** Lock-free counters avoid contention on hot paths while providing real-time
     performance visibility both locally and remotely.
   - **Verification:** Unit test: increment a counter from 8 threads for 1,000,000 iterations;
     verify merged total. Assert increment latency under 50 ns. Verify telemetry snapshots arrive at
     configured intervals.

6. **R-14.4.6** — The engine **SHALL** write incrementing marker values into a GPU-visible buffer
   before and after each significant render pass. When a GPU hang occurs, the last completed marker
   identifies the faulting pass. Breadcrumb data **SHALL** be included in the crash report.
   - **Rationale:** GPU hangs produce no CPU-side stack trace; breadcrumb markers are the primary
     mechanism for identifying the faulting pass.
   - **Verification:** Integration test: inject a GPU hang after a known marker; verify breadcrumb
     buffer reports the last completed marker. Verify the crash report includes CPU dump and GPU
     breadcrumb data.

## Out-of-Process Crash Capture

7. **R-14.4.7** — The engine **SHALL** capture crash dumps in a separate out-of-process monitoring
   binary launched at startup, communicating via pipe or socket, so that capture succeeds even when
   the faulting process's heap or allocator is corrupted.
   - **Rationale:** In-process crash handlers execute in a potentially corrupted address space. An
     out-of-process handler operates cleanly.
   - **Verification:** Integration test per platform: corrupt the heap of a child process, trigger a
     crash, verify the out-of-process handler produces a valid dump. Verify the monitor launches at
     startup and stays connected.

## Crash Dump Management

8. **R-14.4.8** — The engine **SHALL** retain at most a configurable number of crash dumps, rotating
   out the oldest, and **SHALL** attach key-value metadata (build ID, GPU driver, player ID, scene
   name) for server-side filtering without symbolication.
   - **Rationale:** Unbounded accumulation wastes disk space. Metadata enables filtering before
     expensive symbolication.
   - **Verification:** Unit test: configure max 5 dumps, generate 8, verify only 5 most recent
     remain. Attach metadata and verify the aggregation server can filter by each field.

## Log Extensibility

9. **R-14.4.9** — The engine **SHALL** support updating the log filter (minimum severity per
   channel) at runtime without restarting, and **SHALL** accept pluggable log sinks via a `LogSink`
   trait composed at construction time.
   - **Rationale:** Runtime filter updates let developers increase verbosity during live debugging.
     Pluggable sinks decouple output destinations from the logger core.
   - **Verification:** Integration test: set default level to Warn, emit Debug, verify filtered out.
     Update filter to Debug at runtime, re-emit, verify appears. Verify three sink types compose
     simultaneously.

## Platform-Native Log Integration

10. **R-14.4.10** — The engine **SHALL** provide log sinks that emit to OutputDebugString on
    Windows, `os_log` on macOS, and `sd_journal_sendv` on Linux, so that engine logs appear in
    platform-native debugging tools.
    - **Rationale:** Platform-native log sinks let developers view engine output in familiar tools
      (Visual Studio debugger, Console.app, journalctl).
    - **Verification:** Integration test per platform: emit a log record through the platform sink;
      verify it appears in the platform-native log viewer with correct severity and message.

## Platform-Native Profiling Integration

11. **R-14.4.11** — The engine **SHALL** integrate with ETW on Windows, `os_signpost` on macOS, and
    `perf_event_open` on Linux, so that performance counter data appears in platform-native
    profiling tools.
    - **Rationale:** Platform-native profiling tools provide hardware-level visibility that engine
      tools cannot replicate.
    - **Verification:** Integration test per platform: emit performance markers through the platform
      tracing sink; verify they appear in the native profiler (Windows Performance Analyzer,
      Instruments, perf).
