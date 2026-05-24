# Crash and Telemetry

Crash reporting, profiling, and diagnostics.

## What it covers

- Automatic crash capture: faulting thread stack, registers, memory snapshot, GPU breadcrumb
  markers.
- Symbol upload at build time, server-side symbolication and crash aggregation.
- Structured logging with timestamp, severity, channel, and key/value fields.
- Lock-free per-thread performance counters merged at frame boundaries.
- Platform-native log and profiling sink integration.
- Out-of-process crash capture for corrupted-heap scenarios.
- Configurable log-level filtering and pluggable log sinks.
- Optional telemetry relay to remote servers.

## Concepts

### Crash Handler

A process-wide exception handler captures crashes (segfaults, abort, unhandled exceptions)
automatically without user interaction. Crash dumps include the faulting thread's stack trace,
registers, and a snapshot of key memory regions. GPU breadcrumb markers identify which render pass
was executing at the time.

### Symbolication

Debug symbols are uploaded during the build pipeline, indexed by build ID. At crash aggregation
time, the server looks up symbols to convert addresses back to source file and line number. This
enables meaningful crash reporting without shipping debug symbols.

### Structured Logging

Logs are tagged with channel (physics, rendering, network, etc.), severity (debug, info, warning,
error), and key/value fields. An async ring buffer writes to disk; optional relay sends critical
logs to a remote telemetry server. Runtime filters control verbosity per channel.

## How it fits

- See [windowing-and-display](./windowing-and-display.md) for frame-pacing metrics captured in
  performance counters.
- See [threading-and-async](./threading-and-async.md) for lock-free counter increments.
- Integrates with all domains for structured event logging.
