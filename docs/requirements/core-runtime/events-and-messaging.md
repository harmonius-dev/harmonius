# R-1.5 — Events & Messaging Requirements

## Typed Event Channels

| ID       | Derived From                                                   |
|----------|----------------------------------------------------------------|
| R-1.5.1  | [F-1.5.1](../../features/core-runtime/events-and-messaging.md) |
| R-1.5.1a | [F-1.5.1](../../features/core-runtime/events-and-messaging.md) |
| R-1.5.2  | [F-1.5.2](../../features/core-runtime/events-and-messaging.md) |

1. **R-1.5.1** — The engine **SHALL** provide strongly-typed event channels with double buffering,
   where producers write to a back buffer and consumers read from a front buffer swapped at frame
   boundaries, with each event type allocated its own channel to enable contention-free parallel
   reads.
   - **Rationale:** Per-type channels with double buffering eliminate dynamic dispatch and allow
     lock-free parallel event consumption across systems.
   - **Verification:** Unit test: send 3 events of type A and 2 events of type B in frame N. In
     frame N+1, verify readers of type A see exactly 3 events and readers of type B see exactly 2.
     In frame N+2, verify both channels are empty. Stress test: 8 threads reading the same channel
     concurrently with no data races (verify via ThreadSanitizer).
2. **R-1.5.1a** — Each event channel **SHALL** support at least 100,000 events per frame with under
   1 millisecond total write time. Per-channel memory **SHALL NOT** exceed 2x the peak frame event
   payload size (double buffer overhead). The engine **SHALL** report a diagnostic warning if any
   channel exceeds 50,000 events in a single frame, indicating potential event flooding.
   - **Rationale:** Event channels are used for high-volume gameplay events; unbounded growth or
     slow writes degrade frame times and memory budgets.
   - **Verification:** Benchmark: write 100,000 events of 64 bytes each and verify total write time.
     Verify memory usage matches expected double-buffer overhead. Verify the warning fires at 50,001
     events.
3. **R-1.5.2** — The engine **SHALL** support persistent event streams where events are retained
   across frames and each reader maintains an independent cursor, consuming events at its own pace
   without blocking writers.
   - **Rationale:** Systems running at different tick rates (physics at 60 Hz, AI at 10 Hz) must
     process events without loss despite tick-rate differences.
   - **Verification:** Unit test: send 60 events across 6 frames. A reader consuming every 6th frame
     reads all 60 events in a single batch. Verify no events are lost or duplicated. Verify two
     readers with different cursor positions see independent views.

## Observer Pattern

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.5.3 | [F-1.5.3](../../features/core-runtime/events-and-messaging.md) |

1. **R-1.5.3** — The engine **SHALL** invoke observer callbacks for component add, remove, and
   mutate events during command buffer application at sync points, preserving deterministic
   execution order. This requirement extends R-1.1.30 (Event-Triggered Observers) with
   component-specific lifecycle hooks.
   - **Rationale:** Sync-point execution ensures observers can safely perform structural changes
     without conflicting with parallel system iteration.
   - **Verification:** Unit test: register an observer for component A add events. Spawn 100
     entities with component A via command buffers from 4 parallel systems. Flush at a sync point
     and verify the observer fires exactly 100 times in deterministic order across repeated runs.

## Deferred Command Buffers

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.5.4 | [F-1.5.4](../../features/core-runtime/events-and-messaging.md) |

1. **R-1.5.4** — The engine **SHALL** provide per-system command buffers recording world-mutating
   operations (spawn, despawn, insert, remove, send event) for deferred execution at explicit sync
   points in deterministic order matching system execution order. This requirement extends R-1.1.32
   (Deferred Structural Changes via Command Buffers) with inter-system communication patterns.
   - **Rationale:** Deferred execution eliminates exclusive world access requirements during
     parallel system runs.
   - **Verification:** Unit test: record spawn and despawn commands from 3 systems in known
     execution order. Flush and verify operations apply in system execution order. Repeat 10 times
     and verify identical final world state.

## Reactive Queries

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.5.5 | [F-1.5.5](../../features/core-runtime/events-and-messaging.md) |

1. **R-1.5.5** — The engine **SHALL** allow queries to subscribe to archetype-level change
   notifications, automatically skipping system execution when no entities matching the query have
   been modified since the last run.
   - **Rationale:** Skipping unchanged systems reduces per-tick overhead for conditionally-active
     systems in server scenarios.
   - **Verification:** Unit test: register a system with a reactive query on component A. Run 10
     frames without modifying any A component. Verify the system body executes zero times. Modify
     one entity's A component and verify the system executes on the next frame.

## Inter-System Communication

| ID       | Derived From                                                   |
|----------|----------------------------------------------------------------|
| R-1.5.6  | [F-1.5.6](../../features/core-runtime/events-and-messaging.md) |
| R-1.5.5a | [F-1.5.5](../../features/core-runtime/events-and-messaging.md) |
| R-1.5.7  | [F-1.5.7](../../features/core-runtime/events-and-messaging.md) |

1. **R-1.5.6** — The engine **SHALL** expose typed singleton resources within each ECS world that
   participate in the scheduler's dependency analysis, ensuring safe concurrent `Res<T>` (shared)
   and `ResMut<T>` (exclusive) access across systems. This requirement extends R-1.1.23 (World
   Resources) with inter-system shared state patterns.
   - **Rationale:** Resources are the primary communication channel for cross-cutting state and must
     be scheduler-aware to prevent data races.
   - **Verification:** Unit test: create a resource, have one system write via `ResMut<T>` and
     another read via `Res<T>`. Verify the scheduler orders them correctly (writer before reader or
     vice versa). Verify concurrent `Res<T>` access from two read-only systems executes in parallel.
2. **R-1.5.5a** — Reactive query change subscription overhead **SHALL NOT** exceed 1 microsecond per
   query per frame for the no-change case (system skipped). The system **SHALL** produce zero false
   negatives — if a matching entity was modified, the system must run.
   - **Rationale:** Reactive queries are a latency optimization; if the check itself is expensive or
     misses changes, the feature is counterproductive.
   - **Verification:** Benchmark: register 200 reactive queries on a world with no changes. Verify
     total per-frame overhead for all 200 queries is under 200 microseconds. Modify one entity and
     verify the corresponding system runs on the next frame (no false negative).
3. **R-1.5.7** — The engine **SHALL** enable event routing between independent ECS worlds via bridge
   subscriptions that re-publish events from a source world into a target world, with optional
   transformation and filtering in transit.
   - **Rationale:** Instanced zones, lobbies, and the overworld running as separate ECS worlds must
     exchange gameplay events without tight coupling.
   - **Verification:** Integration test: create two worlds with a bridge from world A to world B for
     event type `ChatMessage`. Send a `ChatMessage` in world A and verify it appears in world B's
     event channel. Send an unsubscribed event type in world A and verify it does not appear in
     world B. Configure a filter that drops messages with `is_private=true` and verify filtered
     events are not bridged.
