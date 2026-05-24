# Data and IO

Serialization, memory, and asynchronous file work.

## What it covers

- Binary and text serialization formats for assets, saves, and network replication.
- Schema versioning and automatic migration from old saves to new data layouts.
- Memory-mapped zero-copy deserialization for static datasets.
- Reflection-driven text serialization with round-trip fidelity for human editing.
- Per-frame bump (arena) allocators that reset at frame boundary with zero overhead.
- Fixed-block pool allocators for predictable allocation patterns.
- Generational handles for safe O(1) recycling and stale-reference detection.
- Async file I/O with prioritization (critical, normal, background).
- Page-aligned I/O buffers and scatter-gather operations.

## Concepts

### Serialization

The engine uses compact binary format for performance and human-readable text format for git
editability. Schemas are versioned; old saves automatically migrate through a registry of
version-to-version transforms over dynamic values. Assets are referenced by stable IDs and resolved
asynchronously at load time.

### Memory Allocation

Most frame-scoped allocations use per-frame bump arenas that reset at frame end with no
fragmentation. Longer-lived data uses fixed-block pools or heap allocators with built-in profiling
(dev-only). All allocators tag subsystem ownership; out-of-budget attempts raise errors rather than
failing silently.

### Async IO

File, network, and timer I/O all use the same async facade. I/O is polled once per frame; the engine
never blocks on I/O in hot code paths. Network operations support TCP and UDP. Audio uses dedicated
real-time threads but integrates into the async system.

## How it fits

- See [world-and-entities](./world-and-entities.md) for how entities serialize with ID
  remapping.
- See [authoring-runtime](./authoring-runtime.md) for how scenes mix text metadata with binary
  asset companions.
- Integrates with [platform](../platform/os-services.md) for platform-specific I/O APIs.
