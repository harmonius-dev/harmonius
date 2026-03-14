# 14.4 — Crash Reporting & Diagnostics

## Crash Handling

### F-14.4.1 Crash Handler and Minidump Generation

Install a process-wide crash handler that intercepts unhandled exceptions, segfaults, and
aborts, then writes a minidump (Windows) or core dump (macOS/Linux) containing the faulting
thread's stack, register state, and a snapshot of key memory regions. For a live-service MMO,
every client crash must produce an actionable artifact without requiring the player to do
anything.

- **Requirements:** R-14.4.1
- **Dependencies:** None
- **Platform notes:** Windows uses `SetUnhandledExceptionFilter` and `MiniDumpWriteDump` via
  COM wrappers; macOS uses a Mach exception handler (`mach_port_allocate`,
  `mach_msg_server`) via Objective-C++ cxx wrappers; Linux uses a `SIGSEGV`/`SIGABRT` signal
  handler with `/proc/self/maps` parsing and `ptrace`-based stack capture. Out-of-process
  crash capture is preferred on all platforms to avoid corruption in the faulting process.

### F-14.4.2 Symbol Upload and Server-Side Symbolication

Upload debug symbols (PDB on Windows, dSYM on macOS, DWARF on Linux) to a crash aggregation
service during the build pipeline. Crash reports reference a build ID that the backend uses to
symbolicate stack traces into function names, file paths, and line numbers.

- **Requirements:** R-14.4.2
- **Dependencies:** F-14.4.1
- **Platform notes:** Windows PDB files are indexed by GUID+age from the PE debug directory;
  macOS dSYM bundles are indexed by UUID from `LC_UUID`; Linux uses GNU build-id
  (`.note.gnu.build-id` ELF section). The upload tool is a standalone CLI invoked as a
  post-build CI step.

### F-14.4.3 Crash Aggregation and Alerting

Group incoming crash reports by symbolicated stack signature, track frequency over time, and
alert the development team when a new crash cluster appears or an existing one spikes. The
aggregation backend integrates with the live-ops dashboard so that server-side hotfixes or
client patches can be prioritized by crash volume.

- **Requirements:** R-14.4.3
- **Dependencies:** F-14.4.1, F-14.4.2
- **Platform notes:** None

## Diagnostics

### F-14.4.4 Structured Logging with Severity and Channels

Emit structured log records (timestamp, severity, channel, key-value fields) from any engine
system. Logs are written to a ring buffer and flushed asynchronously to disk and/or a remote
telemetry endpoint. Channel-based filtering allows players and developers to isolate
rendering, networking, or gameplay logs without noise.

- **Requirements:** R-14.4.4
- **Dependencies:** None
- **Platform notes:** Windows OutputDebugString integration for debugger capture; macOS
  `os_log` integration for unified Console.app viewing; Linux `sd_journal_sendv` for
  journald. All platforms share the same structured format; platform log sinks are additive
  alongside the file sink.

### F-14.4.5 Performance Counters and Telemetry Hooks

Expose named counters (frame time, draw calls, entity count, network RTT) that the profiler,
HUD, and telemetry systems can read. Counters are lock-free per-thread accumulators merged at
frame boundaries. A telemetry hook sends periodic snapshots to the live-ops backend for
server-side performance monitoring across the MMO player base.

- **Requirements:** R-14.4.5
- **Dependencies:** None
- **Platform notes:** Windows integrates with ETW (`EventWrite`) for system-wide tracing;
  macOS integrates with `os_signpost` for Instruments profiling; Linux integrates with
  `perf_event_open` for hardware counter access. Platform trace sinks complement the
  engine's internal counters.

### F-14.4.6 GPU Crash Breadcrumbs

Write incrementing marker values into a GPU-visible buffer before and after significant
render passes. When a GPU hang or device-lost event occurs, the last completed marker
identifies which pass caused the fault. Breadcrumb data is included in crash reports
alongside CPU minidumps.

- **Requirements:** R-14.4.6
- **Dependencies:** F-14.4.1
- **Platform notes:** Vulkan uses `VK_AMD_buffer_marker` or `VK_NV_device_diagnostic_checkpoints`;
  Direct3D 12 uses DRED (Device Removed Extended Data) breadcrumbs via
  `ID3D12DeviceRemovedExtendedDataSettings`. Metal has no native breadcrumb API — the engine
  writes to a shared `MTLBuffer` and reads it back on command buffer error callback.
