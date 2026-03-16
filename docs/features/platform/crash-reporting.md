# 14.4 — Crash Reporting & Diagnostics

## Crash Handling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.4.1 | Crash Handler and Minidump Generation | Install a process-wide crash handler that intercepts unhandled exceptions, segfaults, and aborts, then writes a minidump (Windows) or core dump (macOS/Linux) containing the faulting thread's stack, register state, and a snapshot of key memory regions. For a live-service MMO, every client crash must produce an actionable artifact without requiring the player to do anything. | R-14.4.1 | None | Windows uses `SetUnhandledExceptionFilter` and `MiniDumpWriteDump` via COM wrappers; macOS uses a Mach exception handler (`mach_port_allocate`, `mach_msg_server`) via Objective-C++ cxx wrappers; Linux uses a `SIGSEGV`/`SIGABRT` signal handler with `/proc/self/maps` parsing and `ptrace`-based stack capture. Out-of-process crash capture is preferred on all platforms to avoid corruption in the faulting process. |
| F-14.4.2 | Symbol Upload and Server-Side Symbolication | Upload debug symbols (PDB on Windows, dSYM on macOS, DWARF on Linux) to a crash aggregation service during the build pipeline. Crash reports reference a build ID that the backend uses to symbolicate stack traces into function names, file paths, and line numbers. | R-14.4.2 | F-14.4.1 | Windows PDB files are indexed by GUID+age from the PE debug directory; macOS dSYM bundles are indexed by UUID from `LC_UUID`; Linux uses GNU build-id (`.note.gnu.build-id` ELF section). The upload tool is a standalone CLI invoked as a post-build CI step. |
| F-14.4.3 | Crash Aggregation and Alerting | Group incoming crash reports by symbolicated stack signature, track frequency over time, and alert the development team when a new crash cluster appears or an existing one spikes. The aggregation backend integrates with the live-ops dashboard so that server-side hotfixes or client patches can be prioritized by crash volume. | R-14.4.3 | F-14.4.1, F-14.4.2 | None |

## Diagnostics

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.4.4 | Structured Logging with Severity and Channels | Emit structured log records (timestamp, severity, channel, key-value fields) from any engine system. Logs are written to a ring buffer and flushed asynchronously to disk and/or a remote telemetry endpoint. Channel-based filtering allows players and developers to isolate rendering, networking, or gameplay logs without noise. | R-14.4.4 | None | Windows OutputDebugString integration for debugger capture; macOS `os_log` integration for unified Console.app viewing; Linux `sd_journal_sendv` for journald. All platforms share the same structured format; platform log sinks are additive alongside the file sink. |
| F-14.4.5 | Performance Counters and Telemetry Hooks | Expose named counters (frame time, draw calls, entity count, network RTT) that the profiler, HUD, and telemetry systems can read. Counters are lock-free per-thread accumulators merged at frame boundaries. A telemetry hook sends periodic snapshots to the live-ops backend for server-side performance monitoring across the MMO player base. | R-14.4.5 | None | Windows integrates with ETW (`EventWrite`) for system-wide tracing; macOS integrates with `os_signpost` for Instruments profiling; Linux integrates with `perf_event_open` for hardware counter access. Platform trace sinks complement the engine's internal counters. |
| F-14.4.6 | GPU Crash Breadcrumbs | Write incrementing marker values into a GPU-visible buffer before and after significant render passes. When a GPU hang or device-lost event occurs, the last completed marker identifies which pass caused the fault. Breadcrumb data is included in crash reports alongside CPU minidumps. | R-14.4.6 | F-14.4.1 | Vulkan uses `VK_AMD_buffer_marker` or `VK_NV_device_diagnostic_checkpoints`; Direct3D 12 uses DRED (Device Removed Extended Data) breadcrumbs via `ID3D12DeviceRemovedExtendedDataSettings`. Metal has no native breadcrumb API — the engine writes to a shared `MTLBuffer` and reads it back on command buffer error callback. |

## Out-of-Process Crash Capture

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.4.7 | Out-of-Process Crash Handler Binary | Run crash dump capture in a separate monitoring process rather than in the faulting process. The game process launches the out-of-process handler at startup and maintains a pipe or socket connection. When a fault occurs, the in-process signal handler notifies the monitor, which attaches to the faulting process, captures the stack, registers, and memory regions, writes the dump, and terminates the faulting process. This avoids relying on the faulting process's potentially corrupted heap, allocator, or thread state. | R-14.4.7 | F-14.4.1 | Windows uses `MiniDumpWriteDump` called from the monitor process with a handle to the faulting process. macOS uses Mach exception ports forwarded to the monitor via `mach_msg`. Linux uses `ptrace(PTRACE_ATTACH)` from the monitor and `/proc/<pid>/maps` for memory layout. |

## Crash Dump Management

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.4.8 | Crash Dump Rotation and Metadata Attachment | Retain a configurable maximum number of crash dumps in the crash directory, rotating out the oldest dumps when the limit is reached. Attach key-value metadata (build ID, GPU driver version, player ID, scene name) to each crash dump so that the aggregation server can filter and search reports by metadata fields without symbolicating first. | R-14.4.8 | F-14.4.1, F-14.4.7 | None |

## Log Extensibility

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.4.9 | Runtime Log Filter Updates and Pluggable Sinks | Allow the log filter (minimum severity per channel) to be updated at runtime from the developer console or a remote command without restarting the engine. Support pluggable log sinks via a `LogSink` trait so that platform-native sinks (OutputDebugString, os_log, sd_journal), file sinks, and remote telemetry sinks can be composed at construction time via dependency injection. | R-14.4.9 | F-14.4.4 | None |
