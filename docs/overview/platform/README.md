# Platform

Operating system integration, windowing, threading, and crash handling.

## Topics

- [windowing-and-display](./windowing-and-display.md) — windows, fullscreen, resizing, and
  display modes.
- [threading-and-async](./threading-and-async.md) — worker threads, task scheduling, and
  parallelism.
- [os-services](./os-services.md) — file I/O, memory, and OS integration points.
- [crash-and-telemetry](./crash-and-telemetry.md) — crash reports, diagnostics, and
  analytics.

## Key takeaways

- Platform abstraction layer (PAL) unifies Windows, macOS, Linux, console APIs, enabling single
  codebase across platforms.
- Worker thread pool with task scheduling enables CPU parallelism: rendering, simulation, audio,
  and I/O tasks run concurrently.
- Async I/O (file operations off main thread) prevents frame stalls; I/O callbacks notify when
  ready.
- Crash handler captures callstacks, registers, memory dumps enabling diagnosis without shipping
  debug builds.
- Telemetry collects session data (playtime, crashes, perf metrics) enabling data-driven decisions.

## Integration risks

- Platform abstraction layer leaks (platform-specific code outside PAL) compromise portability;
  strict encapsulation necessary. See [windowing-and-display.md](./windowing-and-display.md)
  for module boundaries.
- Worker thread synchronization (locks, atomics) introduces deadlock risks; lock-free data
  structures preferred where feasible. See [threading-and-async.md](./threading-and-async.md)
  for synchronization patterns.
- Async I/O race conditions (resource accessed before loaded) require careful dependency ordering.
  See [os-services.md](./os-services.md) for load ordering.
- Crash handler signal safety (only async-signal-safe functions allowed in signal handlers) requires
  careful coding. See [crash-and-telemetry.md](./crash-and-telemetry.md) for signal-safe
  implementation.
- Telemetry privacy (collecting personally identifiable data) requires consent and compliance with
  privacy regulations. See [crash-and-telemetry.md](./crash-and-telemetry.md) for privacy
  guidelines.
