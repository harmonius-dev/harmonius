# R-1.5 — Events & Messaging Requirements

## Typed Event Channels

1. **R-1.5.1** — The engine **SHALL** provide strongly-typed event channels with double buffering,
   where producers write to a back buffer and consumers read from a front buffer swapped at frame
   boundaries. Each event type **SHALL** get its own channel for contention-free parallel reads.
   - **Rationale:** Per-type channels with double buffering eliminate dynamic dispatch and allow
     lock-free parallel consumption.
   - **Verification:** Send 3 events of type A in frame N. In frame N+1, verify readers see exactly
     3. In frame N+2, verify channel empty. Stress test: 8 threads reading concurrently, no data
     races via ThreadSanitizer.
2. **R-1.5.2** — Each event channel **SHALL** support at least 100,000 events per frame with under 1
   ms total write time. Per-channel memory **SHALL NOT** exceed 2x the peak frame event payload
   size.
   - **Rationale:** High-volume gameplay events must not degrade frame times or memory budgets.
   - **Verification:** Write 100,000 events of 64 bytes each; verify total write time under 1 ms.
     Verify memory matches expected double-buffer overhead.
3. **R-1.5.3** — The engine **SHALL** support persistent event streams where events are retained
   across frames and each reader maintains an independent cursor, consuming at its own pace without
   blocking writers.
   - **Rationale:** Systems at different tick rates (physics 60 Hz, AI 10 Hz) must process events
     without loss.
   - **Verification:** Send 60 events across 6 frames. A reader consuming every 6th frame reads all
     60 in one batch. Verify no events lost or duplicated. Verify two readers see independent views.

## Observer Pattern

1. **R-1.5.4** — The engine **SHALL** invoke observer callbacks for component add, remove, and
   mutate events during command buffer application at sync points, preserving deterministic
   execution order.
   - **Rationale:** Sync-point execution ensures observers can safely perform structural changes.
   - **Verification:** Register observer for component A add. Spawn 100 entities with A from 4
     parallel systems. Flush; verify observer fires exactly 100 times in deterministic order across
     repeated runs.

## Deferred Command Buffers

1. **R-1.5.5** — The engine **SHALL** provide per-system command buffers recording world-mutating
   operations (spawn, despawn, insert, remove, send event) for deferred execution at explicit sync
   points in deterministic order matching system execution order.
   - **Rationale:** Deferred execution eliminates exclusive world access during parallel system
     runs.
   - **Verification:** Record commands from 3 systems in known order; flush; verify operations apply
     in system execution order. Repeat 10 times; verify identical final state.

## Reactive Queries

1. **R-1.5.6** — The engine **SHALL** allow queries to subscribe to archetype-level change
   notifications, automatically skipping system execution when no matching entities have been
   modified since the last run. The system **SHALL** produce zero false negatives.
   - **Rationale:** Skipping unchanged systems reduces per-tick overhead for conditionally-active
     systems.
   - **Verification:** Register reactive query on component A; run 10 frames without modifying A;
     verify system body executes zero times. Modify one A; verify system runs next frame.
2. **R-1.5.7** — Reactive query subscription overhead **SHALL NOT** exceed 1 us per query per frame
   for the no-change case.
   - **Rationale:** The check itself must not negate the savings from skipping idle systems.
   - **Verification:** Register 200 reactive queries with no changes; verify total per-frame
     overhead under 200 us.

## Inter-System Communication

1. **R-1.5.8** — The engine **SHALL** expose typed singleton resources within each ECS world,
   participating in the scheduler's dependency analysis for safe concurrent Res (shared) and ResMut
   (exclusive) access.
   - **Rationale:** Resources are the primary channel for cross-cutting state and must be
     scheduler-aware to prevent data races.
   - **Verification:** Have one system write via ResMut and another read via Res; verify scheduler
     orders them. Verify concurrent Res access from two read-only systems runs in parallel.
2. **R-1.5.9** — The engine **SHALL** enable event routing between independent ECS worlds via bridge
   subscriptions that re-publish events from a source world into a target world, with optional
   transformation and filtering.
   - **Rationale:** Instanced zones and lobbies must exchange events without tight coupling.
   - **Verification:** Create two worlds with bridge for ChatMessage. Send in world A; verify
     appears in B. Send unsubscribed event; verify absent in B. Configure filter blocking private
     messages; verify filtered.

## Event Flood Detection

1. **R-1.5.10** — The engine **SHALL** emit a diagnostic warning when the number of events written
   to a single channel in one frame exceeds a configurable flood threshold.
   - **Rationale:** Runaway event production can exhaust memory and degrade frame times; early
     detection enables debugging before the system degrades.
   - **Verification:** Set flood threshold to 1,000; write 1,001 events; verify diagnostic warning
     emitted naming the event type and count. Write 999; verify no warning.

## Entity Event Propagation

1. **R-1.5.11** — The engine **SHALL** propagate entity events through the hierarchy in two phases:
   capture (root-to-target) and bubble (target-to-root), with the ability to stop propagation at any
   point.
   - **Rationale:** DOM-style capture/bubble propagation enables parent widgets to intercept child
     events and leaf entities to prevent parent handling when appropriate.
   - **Verification:** Emit event at a leaf 4 levels deep; verify capture observers fire root to
     leaf, then bubble observers fire leaf to root. Call stop_propagation at depth 2; verify no
     further observers fire.

## Stream Overflow Detection

1. **R-1.5.12** — Each persistent stream cursor **SHALL** detect when it has fallen behind the ring
   buffer and events were lost, reporting the overflow condition to the reader.
   - **Rationale:** Slow consumers must know when they have missed events to avoid processing a
     partial or inconsistent event sequence.
   - **Verification:** Create a stream with 100-event ring buffer; write 200 events without reading;
     advance cursor; verify has_overflowed() returns true with count of lost events.

## Command Buffer Event Recording

1. **R-1.5.13** — The engine **SHALL** allow command buffers to record event send operations that
   are flushed into the event channel back buffer at sync points alongside structural changes.
   - **Rationale:** Systems that spawn entities and emit events in the same command sequence need
     both operations applied atomically at the sync point.
   - **Verification:** Record spawn + send_event in one command buffer; flush at sync point; verify
     entity exists and event is in the back buffer. Verify event is visible to readers in the next
     frame.
