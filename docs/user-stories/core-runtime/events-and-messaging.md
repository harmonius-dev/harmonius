# Events & Messaging User Stories

## Typed Event Channels

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.5.1 | game developer (P-15) | strongly-typed event channels with double buffering that swap at frame boundaries, so that events are visible to all readers for exactly one frame with no contention between parallel consumers | One channel per event type<br>Back buffer for writing, front buffer for reading<br>Buffers swap at frame boundaries<br>No dynamic dispatch on event read/write | F-1.5.1 | R-1.5.1 |
| US-1.5.2 | engine developer (P-26) | per-type event channels that avoid contention between parallel readers, so that event consumption does not become a bottleneck when many systems read the same event type simultaneously | Parallel reads from front buffer without locks<br>No contention between simultaneous readers<br>Event visibility consistent within a frame | F-1.5.1 | R-1.5.1 |
| US-1.5.3 | engine tester (P-27) | to verify that events written in frame N are visible to all readers in frame N+1 and gone by frame N+2, so that the double-buffer swap semantics are correct and no events leak across frame boundaries | Events written in frame N visible in frame N+1<br>Events not visible in frame N+2<br>No events lost during buffer swap | F-1.5.1 | R-1.5.1 |
| US-1.5.4 | game developer (P-15) | persistent event streams where each reader maintains an independent cursor, so that systems running at different tick rates (physics at 60Hz, AI at 10Hz) consume events at their own pace without loss | Events retained across multiple frames<br>Independent cursor per reader<br>Readers consume at their own pace without blocking writers<br>Platform-appropriate ring buffer caps | F-1.5.2 | R-1.5.2 |
| US-1.5.5 | engine tester (P-27) | to stress-test persistent event streams with readers at varying tick rates and writers at high throughput, so that I can verify no events are lost and cursors advance correctly under contention | 10K events/frame sustained without loss<br>Slow reader (10Hz) catches up correctly<br>Ring buffer wraps without corruption | F-1.5.2 | R-1.5.2 |

## Observer Pattern

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.5.6 | game developer (P-15) | observer callbacks invoked during command buffer application when components are added, removed, or mutated, so that I can implement reactive patterns like spatial index updates with deterministic execution order | Observers fire on add, remove, and mutate<br>Invoked during command buffer application (sync points)<br>Deterministic execution order | F-1.5.3 | R-1.5.3 |
| US-1.5.7 | engine developer (P-26) | to implement observer dispatch that processes lifecycle events at sync points in deterministic order, so that observers can safely perform structural changes without invalidating concurrent iteration | Observers evaluated at designated sync points<br>Safe for structural changes during observer callbacks<br>Order matches command buffer application order | F-1.5.3 | R-1.5.3 |

## Deferred Command Buffers

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.5.8 | engine developer (P-26) | per-system command buffers that record spawn, despawn, insert, remove, and send-event operations for deferred execution at sync points, so that parallel systems record mutations without exclusive world access | Records spawn, despawn, insert, remove, send-event<br>Flushed at explicit sync points<br>Deterministic order matching system execution order | F-1.5.4 | R-1.5.4 |
| US-1.5.9 | engine tester (P-27) | to verify that command buffer flush produces identical world state regardless of parallel system scheduling, so that deterministic simulation is maintained across different hardware and thread configurations | Same world state after flush regardless of system scheduling<br>Order verified across 100+ runs with different thread counts<br>Reproducible on different hardware configurations | F-1.5.4 | R-1.5.4 |

## Reactive Queries

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.5.10 | game developer (P-15) | queries that subscribe to archetype-level change notifications so systems are skipped when no matching entities changed, so that conditionally-active systems incur minimal per-tick overhead when idle | System skipped when no matching entities modified<br>Archetype-level change notifications drive scheduling<br>Overhead reduction measurable on servers with many systems | F-1.5.5 | R-1.5.5 |
| US-1.5.11 | engine tester (P-27) | to benchmark per-tick overhead with reactive query notifications enabled versus disabled on a server with hundreds of systems, so that I can measure the scheduling cost reduction from skipping idle systems | Measurable overhead reduction with reactive queries<br>Idle systems contribute near-zero per-tick cost<br>No false negatives (system skipped when it should run) | F-1.5.5 | R-1.5.5 |

## Inter-System Communication

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.5.12 | game developer (P-15) | typed singleton resources within each ECS world that participate in the scheduler's dependency analysis, so that cross-cutting state like time, input, and configuration is accessed safely across systems | Resources participate in scheduler dependency analysis<br>Safe concurrent access through standard declarations<br>Communication channel for time, input, config | F-1.5.6 | R-1.5.6 |
| US-1.5.13 | designer (P-5) | to inspect world resource values in the visual editor, so that I can see and understand global state like time, input, and configuration that affects system behavior | Resources listed in editor with current values<br>Resource changes reflected in real time<br>Read-only inspection for non-editable resources | F-1.5.6 | R-1.5.6 |
| US-1.5.14 | game developer (P-15) | event bridges that route events between independent ECS worlds with optional transformation and filtering, so that instanced zones, lobbies, and the overworld exchange gameplay events without tight coupling | Bridge subscribes to events in source world<br>Events re-published into target world<br>Optional transformation and filtering in transit | F-1.5.7 | R-1.5.7 |
| US-1.5.15 | engine developer (P-26) | to implement cross-world event bridges that subscribe, filter, optionally transform, and re-publish events between worlds, so that MMO architectures with separate zone worlds can communicate without shared state | Bridge configurable with event type filters<br>Transform function modifies events in transit<br>No shared mutable state between worlds | F-1.5.7 | R-1.5.7 |
| US-1.5.16 | engine tester (P-27) | to verify that event bridge filters correctly block events that should not propagate between worlds, so that isolated zones do not receive irrelevant events from other zones | Filtered events do not appear in target world<br>Allowed events arrive correctly in target world<br>Transform function applied correctly to bridged events | F-1.5.7 | R-1.5.7 |
