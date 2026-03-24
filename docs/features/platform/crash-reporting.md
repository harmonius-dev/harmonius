# 14.4 — Crash Reporting & Diagnostics

## Crash Handling

| ID       | Feature                                     | Requirements |
|----------|---------------------------------------------|--------------|
| F-14.4.1 | Crash Handler and Minidump Generation       | R-14.4.1     |
| F-14.4.2 | Symbol Upload and Server-Side Symbolication | R-14.4.2     |
| F-14.4.3 | Crash Aggregation and Alerting              | R-14.4.3     |

1. **F-14.4.1** — Install a process-wide crash handler that intercepts unhandled exceptions,
   segfaults, and aborts, then writes a minidump (Windows) or core dump (macOS/Linux) containing the
   faulting thread's stack, register state, and a snapshot of key memory regions. For a live-service
   MMO, every client crash must produce an actionable artifact without requiring the player to do
   anything.
   - **Platform:** Windows uses `SetUnhandledExceptionFilter` and `MiniDumpWriteDump` via COM
     wrappers; macOS uses a Mach exception handler (`mach_port_allocate`, `mach_msg_server`) via
     Swift C ABI wrappers; Linux uses a `SIGSEGV`/`SIGABRT` signal handler with
     `/proc/self/maps` parsing and `ptrace`-based stack capture. Out-of-process crash capture is
     preferred on all platforms to avoid corruption in the faulting process.
2. **F-14.4.2** — Upload debug symbols (PDB on Windows, dSYM on macOS, DWARF on Linux) to a crash
   aggregation service during the build pipeline. Crash reports reference a build ID that the
   backend uses to symbolicate stack traces into function names, file paths, and line numbers.
   - **Deps:** F-14.4.1
   - **Platform:** Windows PDB files are indexed by GUID+age from the PE debug directory; macOS dSYM
     bundles are indexed by UUID from `LC_UUID`; Linux uses GNU build-id (`.note.gnu.build-id` ELF
     section). The upload tool is a standalone CLI invoked as a post-build CI step.
3. **F-14.4.3** — Group incoming crash reports by symbolicated stack signature, track frequency over
   time, and alert the development team when a new crash cluster appears or an existing one spikes.
   The aggregation backend integrates with the live-ops dashboard so that server-side hotfixes or
   client patches can be prioritized by crash volume.
   - **Deps:** F-14.4.1, F-14.4.2

## Diagnostics

| ID       | Feature                                       | Requirements |
|----------|-----------------------------------------------|--------------|
| F-14.4.4 | Structured Logging with Severity and Channels | R-14.4.4     |
| F-14.4.5 | Performance Counters and Telemetry Hooks      | R-14.4.5     |
| F-14.4.6 | GPU Crash Breadcrumbs                         | R-14.4.6     |

1. **F-14.4.4** — Emit structured log records (timestamp, severity, channel, key-value fields) from
   any engine system. Logs are written to a ring buffer and flushed asynchronously to disk and/or a
   remote telemetry endpoint. Channel-based filtering allows players and developers to isolate
   rendering, networking, or gameplay logs without noise.
   - **Platform:** Windows OutputDebugString integration for debugger capture; macOS `os_log`
     integration for unified Console.app viewing; Linux `sd_journal_sendv` for journald. All
     platforms share the same structured format; platform log sinks are additive alongside the file
     sink.
2. **F-14.4.5** — Expose named counters (frame time, draw calls, entity count, network RTT) that the
   profiler, HUD, and telemetry systems can read. Counters are lock-free per-thread accumulators
   merged at frame boundaries. A telemetry hook sends periodic snapshots to the live-ops backend for
   server-side performance monitoring across the MMO player base.
   - **Platform:** Windows integrates with ETW (`EventWrite`) for system-wide tracing; macOS
     integrates with `os_signpost` for Instruments profiling; Linux integrates with
     `perf_event_open` for hardware counter access. Platform trace sinks complement the engine's
     internal counters.
3. **F-14.4.6** — Write incrementing marker values into a GPU-visible buffer before and after
   significant render passes. When a GPU hang or device-lost event occurs, the last completed marker
   identifies which pass caused the fault. Breadcrumb data is included in crash reports alongside
   CPU minidumps.
   - **Deps:** F-14.4.1
   - **Platform:** Vulkan uses `VK_AMD_buffer_marker` or `VK_NV_device_diagnostic_checkpoints`;
     Direct3D 12 uses DRED (Device Removed Extended Data) breadcrumbs via
     `ID3D12DeviceRemovedExtendedDataSettings`. Metal has no native breadcrumb API — the engine
     writes to a shared `MTLBuffer` and reads it back on command buffer error callback.

## Out-of-Process Crash Capture

| ID       | Feature                             | Requirements |
|----------|-------------------------------------|--------------|
| F-14.4.7 | Out-of-Process Crash Handler Binary | R-14.4.7     |

1. **F-14.4.7** — Run crash dump capture in a separate monitoring process rather than in the
   faulting process. The game process launches the out-of-process handler at startup and maintains a
   pipe or socket connection. When a fault occurs, the in-process signal handler notifies the
   monitor, which attaches to the faulting process, captures the stack, registers, and memory
   regions, writes the dump, and terminates the faulting process. This avoids relying on the
   faulting process's potentially corrupted heap, allocator, or thread state.
   - **Deps:** F-14.4.1
   - **Platform:** Windows uses `MiniDumpWriteDump` called from the monitor process with a handle to
     the faulting process. macOS uses Mach exception ports forwarded to the monitor via `mach_msg`.
     Linux uses `ptrace(PTRACE_ATTACH)` from the monitor and `/proc/<pid>/maps` for memory layout.

## Crash Dump Management

| ID       | Feature                                     | Requirements |
|----------|---------------------------------------------|--------------|
| F-14.4.8 | Crash Dump Rotation and Metadata Attachment | R-14.4.8     |

1. **F-14.4.8** — Retain a configurable maximum number of crash dumps in the crash directory,
   rotating out the oldest dumps when the limit is reached. Attach key-value metadata (build ID, GPU
   driver version, player ID, scene name) to each crash dump so that the aggregation server can
   filter and search reports by metadata fields without symbolicating first.
   - **Deps:** F-14.4.1, F-14.4.7

## Log Extensibility

| ID       | Feature                                        | Requirements |
|----------|------------------------------------------------|--------------|
| F-14.4.9 | Runtime Log Filter Updates and Pluggable Sinks | R-14.4.9     |

1. **F-14.4.9** — Allow the log filter (minimum severity per channel) to be updated at runtime from
   the developer console or a remote command without restarting the engine. Support pluggable log
   sinks via a `LogSink` trait so that platform-native sinks (OutputDebugString, os_log,
   sd_journal), file sinks, and remote telemetry sinks can be composed at construction time via
   dependency injection.
   - **Deps:** F-14.4.4
