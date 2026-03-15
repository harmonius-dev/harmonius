# R-14.4 — Crash Reporting & Diagnostics Requirements

## Crash Handling

### R-14.4.1 Crash Handler and Minidump Generation

The engine **SHALL** install a process-wide crash handler that intercepts unhandled exceptions,
segfaults, and aborts, then writes a platform-native crash dump (minidump on Windows, core dump on
macOS/Linux) containing the faulting thread's stack, register state, and a snapshot of key memory
regions, without requiring any user interaction.

- **Derived from:** [F-14.4.1](../../features/platform/crash-reporting.md)
- **Rationale:** Every crash must produce an actionable artifact automatically so that players
  never need to perform manual steps to report a crash, and developers receive complete diagnostic
  data for every incident.
- **Verification:** Integration test per platform: trigger a null-pointer dereference in a child
  process, verify the crash handler writes a valid dump file to disk. Verify the dump contains the
  faulting thread's stack and register state by loading it in a platform debugger. Verify
  out-of-process capture produces a valid dump even when the faulting process heap is corrupted.

### R-14.4.2 Symbol Upload and Server-Side Symbolication

The engine **SHALL** upload debug symbols (PDB on Windows, dSYM on macOS, DWARF on Linux) to a
crash aggregation service during the build pipeline, indexed by a build ID embedded in the binary,
enabling server-side symbolication of crash reports into function names, file paths, and line
numbers.

- **Derived from:** [F-14.4.2](../../features/platform/crash-reporting.md)
- **Rationale:** Server-side symbolication decouples crash analysis from local debug symbol
  availability, allowing any team member to diagnose crashes from any build without access to the
  original build machine.
- **Verification:** CI pipeline test: build a release binary, run the upload tool, trigger a crash,
  and verify the aggregation service symbolicates the crash report with correct function names and
  line numbers. Verify each platform's build ID format (PE GUID+age, LC_UUID, GNU build-id) is
  correctly extracted and matched.

### R-14.4.3 Crash Aggregation and Alerting

The engine **SHALL** group incoming crash reports by symbolicated stack signature, track crash
frequency over time, and alert the development team when a new crash cluster appears or an existing
cluster's frequency exceeds a configurable threshold.

- **Derived from:** [F-14.4.3](../../features/platform/crash-reporting.md)
- **Rationale:** Crash clustering and alerting enables rapid triage by surfacing the highest-impact
  crashes first and detecting regression spikes immediately after a patch ships.
- **Verification:** Integration test: submit 50 crash reports with 3 distinct stack signatures.
  Verify the aggregation service groups them into 3 clusters. Inject a spike of 20 reports for one
  cluster within 5 minutes and verify an alert fires. Verify cluster data is accessible from the
  live-ops dashboard.

## Diagnostics

### R-14.4.4 Structured Logging with Severity and Channels

The engine **SHALL** emit structured log records containing timestamp, severity level, channel name,
and key-value fields, written to an async ring buffer and flushed to disk and/or a remote telemetry
endpoint without blocking the calling thread.

- **Derived from:** [F-14.4.4](../../features/platform/crash-reporting.md)
- **Rationale:** Structured logging with channel-based filtering allows developers and players to
  isolate logs from specific subsystems (rendering, networking, gameplay) without noise, while async
  flushing prevents logging from impacting frame time.
- **Verification:** Unit test: emit 10,000 log records across 4 channels at mixed severities.
  Verify all records appear in the output with correct timestamps, severity, channel, and fields.
  Verify filtering by channel and severity excludes non-matching records. Benchmark: verify log
  emission does not block the calling thread for more than 1 microsecond per record.

### R-14.4.5 Performance Counters and Telemetry Hooks

The engine **SHALL** expose named performance counters (frame time, draw calls, entity count,
network RTT) as lock-free per-thread accumulators merged at frame boundaries, readable by the
profiler, HUD, and telemetry systems, with periodic snapshots sent to the live-ops backend.

- **Derived from:** [F-14.4.5](../../features/platform/crash-reporting.md)
- **Rationale:** Lock-free per-thread counters avoid contention on hot paths while providing
  real-time performance visibility both locally (profiler, HUD) and remotely (live-ops monitoring).
- **Verification:** Unit test: increment a counter from 8 threads concurrently for 1,000,000
  iterations. Verify the merged value equals the expected total. Benchmark: verify counter
  increment latency is under 50 nanoseconds. Integration test: verify telemetry snapshots arrive at
  the backend endpoint at the configured interval with correct counter values.

### R-14.4.6 GPU Crash Breadcrumbs

The engine **SHALL** write incrementing marker values into a GPU-visible buffer before and after
each significant render pass, such that when a GPU hang or device-lost event occurs, the last
completed marker identifies which pass caused the fault, and include this breadcrumb data in the
crash report alongside the CPU minidump.

- **Derived from:** [F-14.4.6](../../features/platform/crash-reporting.md)
- **Rationale:** GPU hangs produce no CPU-side stack trace; breadcrumb markers are the primary
  mechanism for identifying which render pass triggered a device-lost event.
- **Verification:** Integration test: inject a deliberate GPU hang (infinite loop shader) after a
  known marker. Verify the breadcrumb buffer reports the last completed marker matching the pass
  before the hang. Verify the crash report includes both the CPU minidump and the GPU breadcrumb
  data.
