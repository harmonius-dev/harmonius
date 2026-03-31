# Events & Messaging User Stories

## Typed Event Channels

| ID       | Persona                 |
|----------|-------------------------|
| US-1.5.1 | game developer (P-15)   |
| US-1.5.2 | engine developer (P-26) |
| US-1.5.3 | engine tester (P-27)    |
| US-1.5.4 | game developer (P-15)   |
| US-1.5.5 | engine tester (P-27)    |

1. **US-1.5.1** — **As a** game developer (P-15), **I want** strongly-typed event channels with
   double buffering that swap at frame boundaries, **so that** events are visible to all readers for
   exactly one frame with no contention between parallel consumers.
2. **US-1.5.2** — **As an** engine developer (P-26), **I want** per-type event channels that avoid
   contention between parallel readers, **so that** event consumption does not become a bottleneck
   when many systems read the same event type simultaneously.
3. **US-1.5.3** — **As an** engine tester (P-27), **I want** to verify that events written in frame
   N are visible in frame N+1 and gone by frame N+2, **so that** the double-buffer swap semantics
   are correct and no events leak across frame boundaries.
4. **US-1.5.4** — **As a** game developer (P-15), **I want** persistent event streams where each
   reader maintains an independent cursor, **so that** systems running at different tick rates
   consume events at their own pace without loss.
5. **US-1.5.5** — **As an** engine tester (P-27), **I want** to stress-test persistent event streams
   with readers at varying tick rates and writers at high throughput, **so that** I can verify no
   events are lost and cursors advance correctly under contention.

## Observer Pattern

| ID       | Persona                 |
|----------|-------------------------|
| US-1.5.6 | game developer (P-15)   |
| US-1.5.7 | engine developer (P-26) |

1. **US-1.5.6** — **As a** game developer (P-15), **I want** observer callbacks invoked during
   command buffer application when components are added, removed, or mutated, **so that** I can
   implement reactive patterns like spatial index updates with deterministic order.
2. **US-1.5.7** — **As an** engine developer (P-26), **I want** observer dispatch to process
   lifecycle events at sync points in deterministic order, **so that** observers can safely perform
   structural changes without invalidating concurrent iteration.

## Deferred Command Buffers

| ID       | Persona                 |
|----------|-------------------------|
| US-1.5.8 | engine developer (P-26) |
| US-1.5.9 | engine tester (P-27)    |

1. **US-1.5.8** — **As an** engine developer (P-26), **I want** per-system command buffers that
   record spawn, despawn, insert, remove, and send-event operations for deferred execution at sync
   points, **so that** parallel systems record mutations without exclusive world access.
2. **US-1.5.9** — **As an** engine tester (P-27), **I want** to verify that command buffer flush
   produces identical world state regardless of parallel system scheduling, **so that**
   deterministic simulation is maintained across different hardware configurations.

## Reactive Queries

| ID        | Persona               |
|-----------|-----------------------|
| US-1.5.10 | game developer (P-15) |
| US-1.5.11 | engine tester (P-27)  |

1. **US-1.5.10** — **As a** game developer (P-15), **I want** queries that subscribe to
   archetype-level change notifications so systems are skipped when no matching entities changed,
   **so that** conditionally active systems incur minimal per-tick overhead when idle.
2. **US-1.5.11** — **As an** engine tester (P-27), **I want** to benchmark per-tick overhead with
   reactive query notifications on a server with hundreds of systems, **so that** I can measure the
   scheduling cost reduction from skipping idle systems.

## Inter-System Communication

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.5.12 | game developer (P-15)   |
| US-1.5.13 | game developer (P-15)   |
| US-1.5.14 | engine developer (P-26) |
| US-1.5.15 | engine tester (P-27)    |

1. **US-1.5.12** — **As a** game developer (P-15), **I want** typed singleton resources within each
   ECS world that participate in the scheduler's dependency analysis, **so that** cross-cutting
   state like time, input, and configuration is accessed safely.
2. **US-1.5.13** — **As a** game developer (P-15), **I want** event bridges that route events
   between independent ECS worlds with optional transformation and filtering, **so that** instanced
   zones and lobbies exchange gameplay events without tight coupling.
3. **US-1.5.14** — **As an** engine developer (P-26), **I want** cross-world event bridges
   configurable with type filters and transform functions, **so that** zone worlds communicate
   without shared mutable state.
4. **US-1.5.15** — **As an** engine tester (P-27), **I want** to verify that event bridge filters
   correctly block events that should not propagate between worlds, **so that** isolated zones do
   not receive irrelevant events from other zones.
