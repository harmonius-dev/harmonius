# User Stories -- 14.4 Crash Reporting & Diagnostics

## US-14.4.1 Report Crashes Automatically Without Player Intervention

**As a** player (P-23), **I want** crashes to automatically generate a dump file and upload
it on next launch, **so that** I do not need to do anything to help developers fix the
problem.

## US-14.4.2 Get Symbolicated Crash Stacks for Every Build

**As a** DevOps engineer (P-16), **I want** debug symbols uploaded during the build pipeline
and crash reports symbolicated server-side using build-ID matching, **so that** every crash
report shows function names, file paths, and line numbers for immediate debugging.

## US-14.4.3 Receive Alerts When a New Crash Cluster Appears

**As a** DevOps engineer (P-16), **I want** crash aggregation that groups reports by stack
signature, tracks frequency, and alerts the team on new clusters or spikes, **so that** we
can prioritize fixes by crash volume in the live-ops dashboard.

## US-14.4.4 Filter Logs by Severity and System

**As a** game developer (P-15), **I want** structured log records with timestamp, severity,
and channel-based filtering (rendering, networking, gameplay), **so that** I can isolate
the log output of a specific system without noise from others.

## US-14.4.5 Monitor Performance Metrics Across the Player Base

**As a** server admin (P-22), **I want** named performance counters (frame time, draw calls,
entity count, network RTT) sent to the live-ops backend via telemetry hooks, **so that** I
can monitor performance distributions across the entire MMO player base in real time.

## US-14.4.6 Diagnose GPU Hangs with Breadcrumb Data

**As an** engine developer (P-26), **I want** GPU crash breadcrumbs that write marker values
before and after each render pass, **so that** when a GPU hang or device-lost event occurs,
the last completed marker identifies the faulting pass in the crash report.

## US-14.4.7 Integrate with Platform-Specific Trace Tools

**As an** engine developer (P-26), **I want** integration with ETW on Windows, os_signpost
on macOS, and perf_event_open on Linux, **so that** engine performance data appears in
platform-native profiling tools (Windows Performance Analyzer, Instruments, perf).

## US-14.4.8 Implement Out-of-Process Crash Capture

**As an** engine developer (P-26), **I want** out-of-process crash capture using platform
APIs (MiniDumpWriteDump, Mach exception handler, ptrace), **so that** the crash handler
produces a reliable dump even when the faulting process has corrupted its own heap.

## US-14.4.9 Use Platform Logging Sinks Alongside File Logs

**As a** game developer (P-15), **I want** structured logs to also emit to OutputDebugString
on Windows, os_log on macOS, and sd_journal on Linux, **so that** I can view engine logs in
platform-native tools (debugger output, Console.app, journalctl).

## US-14.4.10 Verify Crash Handler Produces Valid Dumps on All Platforms

**As an** engine tester (P-27), **I want** automated tests that trigger intentional crashes
(segfault, abort, unhandled exception) and verify a valid minidump or core dump is produced
on each platform, **so that** crash reporting reliability is validated in CI.

## US-14.4.11 Verify Performance Counters Are Lock-Free Under Contention

**As an** engine tester (P-27), **I want** benchmarks that increment performance counters
from all worker threads simultaneously and verify zero contention overhead, **so that**
counter accumulation does not introduce frame time variance under high thread counts.

## US-14.4.12 Verify GPU Breadcrumbs Identify the Correct Faulting Pass

**As an** engine tester (P-27), **I want** tests that intentionally trigger GPU hangs and
verify the breadcrumb marker correctly identifies the injected fault, **so that** GPU crash
diagnostics are validated for each graphics API (Vulkan, D3D12, Metal).
