# Events & Messaging User Stories

## Typed Event Channels

## US-1.5.1 Send Events via Double-Buffered Typed Channels

**As a** game developer (P-15), **I want** strongly-typed event channels with double buffering
that swap at frame boundaries, **so that** events are visible to all readers for exactly one
frame with no contention between parallel consumers.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| One channel per event type | F-1.5.1 | R-1.5.1 |
| Back buffer for writing, front buffer for reading | F-1.5.1 | R-1.5.1 |
| Buffers swap at frame boundaries | F-1.5.1 | R-1.5.1 |
| No dynamic dispatch on event read/write | F-1.5.1 | R-1.5.1 |

## US-1.5.2 Implement Typed Event Channels With Zero Contention

**As an** engine developer (P-26), **I want** per-type event channels that avoid contention
between parallel readers, **so that** event consumption does not become a bottleneck when many
systems read the same event type simultaneously.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Parallel reads from front buffer without locks | F-1.5.1 | R-1.5.1 |
| No contention between simultaneous readers | F-1.5.1 | R-1.5.1 |
| Event visibility consistent within a frame | F-1.5.1 | R-1.5.1 |

## US-1.5.3 Verify Event Channel Frame Boundary Behavior

**As an** engine tester (P-27), **I want** to verify that events written in frame N are
visible to all readers in frame N+1 and gone by frame N+2, **so that** the double-buffer
swap semantics are correct and no events leak across frame boundaries.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Events written in frame N visible in frame N+1 | F-1.5.1 | R-1.5.1 |
| Events not visible in frame N+2 | F-1.5.1 | R-1.5.1 |
| No events lost during buffer swap | F-1.5.1 | R-1.5.1 |

## US-1.5.4 Consume Events at Different Tick Rates via Cursors

**As a** game developer (P-15), **I want** persistent event streams where each reader
maintains an independent cursor, **so that** systems running at different tick rates (physics
at 60Hz, AI at 10Hz) consume events at their own pace without loss.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Events retained across multiple frames | F-1.5.2 | R-1.5.2 |
| Independent cursor per reader | F-1.5.2 | R-1.5.2 |
| Readers consume at their own pace without blocking writers | F-1.5.2 | R-1.5.2 |
| Platform-appropriate ring buffer caps | F-1.5.2 | R-1.5.2 |

## US-1.5.5 Stress-Test Event Streams Under High Throughput

**As an** engine tester (P-27), **I want** to stress-test persistent event streams with
readers at varying tick rates and writers at high throughput, **so that** I can verify no
events are lost and cursors advance correctly under contention.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 10K events/frame sustained without loss | F-1.5.2 | R-1.5.2 |
| Slow reader (10Hz) catches up correctly | F-1.5.2 | R-1.5.2 |
| Ring buffer wraps without corruption | F-1.5.2 | R-1.5.2 |

## Observer Pattern

## US-1.5.6 React to Component Lifecycle Changes via Observers

**As a** game developer (P-15), **I want** observer callbacks invoked during command buffer
application when components are added, removed, or mutated, **so that** I can implement
reactive patterns like spatial index updates with deterministic execution order.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Observers fire on add, remove, and mutate | F-1.5.3 | R-1.5.3 |
| Invoked during command buffer application (sync points) | F-1.5.3 | R-1.5.3 |
| Deterministic execution order | F-1.5.3 | R-1.5.3 |

## US-1.5.7 Implement Observer Dispatch at Sync Points

**As an** engine developer (P-26), **I want** to implement observer dispatch that processes
lifecycle events at sync points in deterministic order, **so that** observers can safely
perform structural changes without invalidating concurrent iteration.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Observers evaluated at designated sync points | F-1.5.3 | R-1.5.3 |
| Safe for structural changes during observer callbacks | F-1.5.3 | R-1.5.3 |
| Order matches command buffer application order | F-1.5.3 | R-1.5.3 |

## Deferred Command Buffers

## US-1.5.8 Defer World Mutations for Deterministic Replay

**As an** engine developer (P-26), **I want** per-system command buffers that record spawn,
despawn, insert, remove, and send-event operations for deferred execution at sync points,
**so that** parallel systems record mutations without exclusive world access.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Records spawn, despawn, insert, remove, send-event | F-1.5.4 | R-1.5.4 |
| Flushed at explicit sync points | F-1.5.4 | R-1.5.4 |
| Deterministic order matching system execution order | F-1.5.4 | R-1.5.4 |

## US-1.5.9 Verify Command Buffer Flush Ordering Is Deterministic

**As an** engine tester (P-27), **I want** to verify that command buffer flush produces
identical world state regardless of parallel system scheduling, **so that** deterministic
simulation is maintained across different hardware and thread configurations.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Same world state after flush regardless of system scheduling | F-1.5.4 | R-1.5.4 |
| Order verified across 100+ runs with different thread counts | F-1.5.4 | R-1.5.4 |
| Reproducible on different hardware configurations | F-1.5.4 | R-1.5.4 |

## Reactive Queries

## US-1.5.10 Skip Systems When No Matching Entities Changed

**As a** game developer (P-15), **I want** queries that subscribe to archetype-level change
notifications so systems are skipped when no matching entities changed, **so that**
conditionally-active systems incur minimal per-tick overhead when idle.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| System skipped when no matching entities modified | F-1.5.5 | R-1.5.5 |
| Archetype-level change notifications drive scheduling | F-1.5.5 | R-1.5.5 |
| Overhead reduction measurable on servers with many systems | F-1.5.5 | R-1.5.5 |

## US-1.5.11 Benchmark Reactive Query Savings on Server Ticks

**As an** engine tester (P-27), **I want** to benchmark per-tick overhead with reactive query
notifications enabled versus disabled on a server with hundreds of systems, **so that** I can
measure the scheduling cost reduction from skipping idle systems.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Measurable overhead reduction with reactive queries | F-1.5.5 | R-1.5.5 |
| Idle systems contribute near-zero per-tick cost | F-1.5.5 | R-1.5.5 |
| No false negatives (system skipped when it should run) | F-1.5.5 | R-1.5.5 |

## Inter-System Communication

## US-1.5.12 Share Cross-Cutting State via Typed Resources

**As a** game developer (P-15), **I want** typed singleton resources within each ECS world
that participate in the scheduler's dependency analysis, **so that** cross-cutting state like
time, input, and configuration is accessed safely across systems.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Resources participate in scheduler dependency analysis | F-1.5.6 | R-1.5.6 |
| Safe concurrent access through standard declarations | F-1.5.6 | R-1.5.6 |
| Communication channel for time, input, config | F-1.5.6 | R-1.5.6 |

## US-1.5.13 Understand Resources as Global State in the Visual Editor

**As a** designer (P-5), **I want** to inspect world resource values in the visual editor,
**so that** I can see and understand global state like time, input, and configuration that
affects system behavior.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Resources listed in editor with current values | F-1.5.6 | R-1.5.6 |
| Resource changes reflected in real time | F-1.5.6 | R-1.5.6 |
| Read-only inspection for non-editable resources | F-1.5.6 | R-1.5.6 |

## US-1.5.14 Route Events Between Independent ECS Worlds

**As a** game developer (P-15), **I want** event bridges that route events between independent
ECS worlds with optional transformation and filtering, **so that** instanced zones, lobbies,
and the overworld exchange gameplay events without tight coupling.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bridge subscribes to events in source world | F-1.5.7 | R-1.5.7 |
| Events re-published into target world | F-1.5.7 | R-1.5.7 |
| Optional transformation and filtering in transit | F-1.5.7 | R-1.5.7 |

## US-1.5.15 Implement Cross-World Event Bridges With Filtering

**As an** engine developer (P-26), **I want** to implement cross-world event bridges that
subscribe, filter, optionally transform, and re-publish events between worlds, **so that**
MMO architectures with separate zone worlds can communicate without shared state.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bridge configurable with event type filters | F-1.5.7 | R-1.5.7 |
| Transform function modifies events in transit | F-1.5.7 | R-1.5.7 |
| No shared mutable state between worlds | F-1.5.7 | R-1.5.7 |

## US-1.5.16 Verify Event Bridge Filtering Prevents Unwanted Propagation

**As an** engine tester (P-27), **I want** to verify that event bridge filters correctly block
events that should not propagate between worlds, **so that** isolated zones do not receive
irrelevant events from other zones.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Filtered events do not appear in target world | F-1.5.7 | R-1.5.7 |
| Allowed events arrive correctly in target world | F-1.5.7 | R-1.5.7 |
| Transform function applied correctly to bridged events | F-1.5.7 | R-1.5.7 |
