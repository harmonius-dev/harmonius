# 1.5 — Events & Messaging

## Typed Event Channels

### F-1.5.1 Typed Double-Buffered Event Channels

Provide strongly-typed event channels where producers write events into a back buffer and consumers read from a
front buffer. Buffers are swapped at frame boundaries so that events are visible to all readers for exactly one
frame. Each event type gets its own channel, avoiding dynamic dispatch and enabling parallel reads without
contention.

- **Requirements:** R-1.5.1
- **Dependencies:** F-1.3.1 (Type Registry)
- **Platform notes:** None

### F-1.5.2 Persistent Event Streams with Cursor-Based Reading

Support persistent event streams where events are retained across multiple frames and each reader maintains an
independent cursor. Readers consume events at their own pace without blocking writers. This enables systems that
run at different tick rates (e.g., physics at 60Hz, AI at 10Hz) to process events without loss, which is essential
for server-authoritative MMO simulation.

- **Requirements:** R-1.5.2
- **Dependencies:** F-1.5.1
- **Platform notes:** Mobile: ring buffer capped at 4K events per channel, 64 max channels.
  Switch: 8K events per channel, 128 channels. Desktop: 32K events per channel,
  configurable channels. Retention window reduced on memory-constrained platforms.

## Observer Pattern

### F-1.5.3 Component Lifecycle Observers

Register observer callbacks that fire when specific components are added, removed, or mutated on any entity.
Observers are invoked during command buffer application at sync points, not immediately on mutation, to preserve
deterministic execution order. This enables reactive patterns like updating spatial indices when transforms change
or triggering network replication when game state is modified.

Extends the observer system defined in F-1.1.30.

- **Requirements:** R-1.5.3
- **Dependencies:** F-1.1.22 (Tick-Based Change Detection),
  F-1.1.32 (Deferred Structural Changes via Command Buffers),
  F-1.1.30 (Event-Triggered Observers)
- **Platform notes:** None

## Deferred Command Buffers

### F-1.5.4 Deferred Command Buffers

Provide per-system command buffers that record world-mutating operations (spawn, despawn, insert component, remove
component, send event) for deferred execution. Commands are flushed at explicit sync points in a deterministic
order matching system execution order. Deferred execution eliminates the need for exclusive world access during
parallel system runs.

Extends the command buffer system defined in F-1.1.32.

- **Requirements:** R-1.5.4
- **Dependencies:** F-1.1.11 (Entity Lifecycle with Generational Indices),
  F-1.7.1 (Arena Allocators), F-1.1.32 (Deferred Structural Changes via Command Buffers)
- **Platform notes:** None

## Reactive Queries

### F-1.5.5 Reactive Query Notifications

Allow queries to subscribe to archetype-level change notifications so that systems are only scheduled when their
query results may have changed. If no entities matching a query were modified since the last run, the system is
skipped entirely. For MMO servers with many conditionally-active systems, this significantly reduces per-tick
overhead.

- **Requirements:** R-1.5.5
- **Dependencies:** F-1.1.17 (Composable Archetype Queries),
  F-1.1.22 (Tick-Based Change Detection)
- **Platform notes:** None

## Inter-System Communication

### F-1.5.6 Typed Singleton Resources for Shared State

Expose globally-unique typed resources (singletons) within each ECS world that systems can read or write through
the standard access-declaration mechanism. Resources participate in the scheduler's dependency analysis, ensuring
safe concurrent access. Resources serve as the communication channel for cross-cutting state like time, input, and
configuration.

Extends the world resources defined in F-1.1.23.

- **Requirements:** R-1.5.6
- **Dependencies:** F-1.1.25 (Dependency Resolution and Topological Ordering),
  F-1.3.1 (Type Registry), F-1.1.23 (World Resources)
- **Platform notes:** None

### F-1.5.7 Cross-World Event Bridges

Enable event routing between independent ECS worlds within the same process. A bridge subscribes to events in a
source world and re-publishes them into a target world, optionally transforming or filtering them in transit. This
supports MMO architectures where instanced zones, lobby worlds, and the overworld run as separate ECS worlds but
must exchange gameplay events.

- **Requirements:** R-1.5.7
- **Dependencies:** F-1.5.1, F-1.1.34 (Multiple World Support)
- **Platform notes:** None
