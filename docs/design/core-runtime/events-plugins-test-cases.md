# Events & Plugins Test Cases

Companion test cases for [events-plugins.md](events-plugins.md).

## Unit Tests

### TC-1.5.1.1 Double Buffer Swap

| # | Requirement |
|---|-------------|
| 1 | R-1.5.1     |
| 2 | R-1.5.1     |
| 3 | R-1.5.1     |

1. **#1** — Write 3 events in frame N
   - **Expected:** Writer buffer has 3 events
2. **#2** — Advance to frame N+1, read
   - **Expected:** Reader sees 3 events
3. **#3** — Advance to frame N+2, read
   - **Expected:** Reader sees 0 events

### TC-1.5.1.2 Parallel Readers No Contention

| # | Requirement |
|---|-------------|
| 1 | R-1.5.1     |
| 2 | R-1.5.1     |

1. **#1** — 8 threads read same channel concurrently
   - **Expected:** All 8 read identical data
2. **#2** — Run under ThreadSanitizer
   - **Expected:** Zero data races

### TC-1.5.1.3 Flood Warning Threshold

| # | Requirement |
|---|-------------|
| 1 | R-1.5.1a    |

1. **#1** — Write 50,001 events in one frame
   - **Expected:** Diagnostic warning fires

### TC-1.5.1.4 Throughput 100K Events

| # | Requirement |
|---|-------------|
| 1 | R-1.5.1a    |

1. **#1** — Write 100,000 events of 64 bytes each
   - **Expected:** Total time < 1 ms

### TC-1.5.2.1 Persistent Stream Cursor

| # | Requirement |
|---|-------------|
| 1 | R-1.5.2     |
| 2 | R-1.5.2     |

1. **#1** — Emit 60 events across 6 frames (10/frame)
   - **Expected:** Stream has 60 events
2. **#2** — Reader at 10 Hz reads batch
   - **Expected:** Sees all 60 events

### TC-1.5.2.2 Cursor Independence

| # | Requirement |
|---|-------------|
| 1 | R-1.5.2     |

1. **#1** — Cursor A at position 10, Cursor B at position 30
   - **Expected:** A sees events 10+, B sees events 30+

### TC-1.5.2.3 Cursor Overflow Detection

| # | Requirement |
|---|-------------|
| 1 | R-1.5.2     |

1. **#1** — Cursor falls behind ring buffer wraparound
   - **Expected:** `has_overflowed()` returns true

### TC-1.5.3.1 Observer Fires OnAdd

| # | Requirement |
|---|-------------|
| 1 | R-1.5.3     |
| 2 | R-1.5.3     |

1. **#1** — Register OnAdd observer, spawn 100 entities via 4 systems
   - **Expected:** 100 callbacks received
2. **#2** — Verify callback order
   - **Expected:** Deterministic across runs

### TC-1.5.3.2 Observer Fires OnRemove

| # | Requirement |
|---|-------------|
| 1 | R-1.5.3     |

1. **#1** — Remove component from 50 entities
   - **Expected:** 50 OnRemove callbacks fired

### TC-1.5.3.3 Observer Fires OnMutate

| # | Requirement |
|---|-------------|
| 1 | R-1.5.3     |

1. **#1** — Mutate component on 25 entities
   - **Expected:** 25 OnMutate callbacks fired

### TC-1.5.3.4 Observer Deterministic Order

| # | Requirement |
|---|-------------|
| 1 | R-1.5.3     |

1. **#1** — Repeat observer test 100 iterations
   - **Expected:** Identical callback order each time

### TC-1.5.4.1 Command Buffer Flush Order

| # | Requirement |
|---|-------------|
| 1 | R-1.5.4     |

1. **#1** — 3 systems record commands in known order
   - **Expected:** Flush applies in system execution order

### TC-1.5.4.2 Command Buffer Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-1.5.4     |

1. **#1** — Repeat flush 100 times with varying thread counts
   - **Expected:** Identical world state each time

### TC-1.5.5.1 Reactive Query Skip

| # | Requirement |
|---|-------------|
| 1 | R-1.5.5     |
| 2 | R-1.5.5     |

1. **#1** — Reactive query on component A, 10 frames no changes
   - **Expected:** System runs 0 times
2. **#2** — Modify one A value
   - **Expected:** System runs next frame

### TC-1.5.5.2 Reactive Query Overhead

| # | Requirement |
|---|-------------|
| 1 | R-1.5.5a    |

1. **#1** — 200 reactive queries, no changes
   - **Expected:** Total check overhead < 200 us

### TC-1.5.6.1 Resource Scheduler Ordering

| # | Requirement |
|---|-------------|
| 1 | R-1.5.6     |

1. **#1** — System A writes via ResMut, System B reads via Res
   - **Expected:** Scheduler orders A before B

### TC-1.5.6.2 Resource Parallel Reads

| # | Requirement |
|---|-------------|
| 1 | R-1.5.6     |

1. **#1** — Two read-only systems with `Res<T>`
   - **Expected:** Both run in parallel

### TC-1.6.1.1 Plugin Reverse Order Registration

| # | Requirement |
|---|-------------|
| 1 | R-1.6.1     |

1. **#1** — Register plugins C, B, A (C depends on B depends on A)
   - **Expected:** Init order: A, B, C

### TC-1.6.1.2 Plugin Contributions

| # | Requirement |
|---|-------------|
| 1 | R-1.6.1     |

1. **#1** — Plugin declares 2 systems, 1 component, 1 resource
   - **Expected:** After init, all 4 contributions exist

### TC-1.6.2.1 Group Disable

| # | Requirement |
|---|-------------|
| 1 | R-1.6.2     |

1. **#1** — Group of 5 plugins, disable plugin 3
   - **Expected:** 4 active, plugin 3's systems absent

### TC-1.6.3.1 Missing Dependency

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |

1. **#1** — Plugin X depends on absent plugin Y
   - **Expected:** Error message names "Y"

### TC-1.6.3.2 Conflict Detection

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |

1. **#1** — Register plugin A and B that declare conflict
   - **Expected:** Conflict error with both names

### TC-1.6.4.1 Topological Sort

| # | Requirement |
|---|-------------|
| 1 | R-1.6.4     |

1. **#1** — A -> B -> C dependency chain
   - **Expected:** Init order: A, B, C

### TC-1.6.4.2 Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-1.6.4     |

1. **#1** — A -> B -> A cycle
   - **Expected:** Error with cycle path [A, B, A]

### TC-1.6.4.3 All Errors Single Pass

| # | Requirement |
|---|-------------|
| 1 | R-1.6.4a    |

1. **#1** — Missing dep + conflict + cycle simultaneously
   - **Expected:** All 3 errors reported in one pass

### TC-1.6.6.1 ABI Hash Match

| # | Requirement |
|---|-------------|
| 1 | R-1.6.6     |

1. **#1** — Load plugin with matching ABI hash
   - **Expected:** Load succeeds

### TC-1.6.6.2 ABI Hash Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-1.6.6     |

1. **#1** — Load plugin with mismatched ABI hash
   - **Expected:** Rejection error with version info

### TC-1.6.7.1 Capability Query

| # | Requirement |
|---|-------------|
| 1 | R-1.6.7     |
| 2 | R-1.6.7     |

1. **#1** — Register capability "physics" v1.2
   - **Expected:** Query "physics" returns v1.2
2. **#2** — Query "audio" (not registered)
   - **Expected:** Returns None

### TC-1.6.7.2 Capability Branch

| # | Requirement |
|---|-------------|
| 1 | R-1.6.7     |
| 2 | R-1.6.7     |

1. **#1** — System branches on "physics" presence (registered)
   - **Expected:** Takes physics-present branch
2. **#2** — System branches on "physics" presence (absent)
   - **Expected:** Takes physics-absent branch

### TC-1.1.1.3 Event Channel Preserves SoA Direct Access

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1a    |
| 2 | R-1.1.1a    |

1. **#1** — Send 10K events referencing component rows without copying data into messages
   - **Expected:** System still iterates archetype columns by reference, no message-copy of SoA
     storage
2. **#2** — Run ThreadSanitizer during send+receive phase
   - **Expected:** Zero data races, archetype columns borrowed directly as in R-1.1.1a

## Integration Tests

### TC-1.5.7.I1 Cross World Bridge

| # | Requirement |
|---|-------------|
| 1 | R-1.5.7     |
| 2 | R-1.5.7     |

1. **#1** — Two worlds A and B, bridge for ChatMsg type
   - **Expected:** Bridge registered
2. **#2** — Send ChatMsg in world A
   - **Expected:** ChatMsg appears in world B

### TC-1.5.7.I2 Bridge Filter

| # | Requirement |
|---|-------------|
| 1 | R-1.5.7     |
| 2 | R-1.5.7     |

1. **#1** — Filter drops events where `is_private=true`
   - **Expected:** Send private + public messages
2. **#2** — Read in target world
   - **Expected:** Only public messages present

### TC-1.5.7.I3 Bridge Transform

| # | Requirement |
|---|-------------|
| 1 | R-1.5.7     |
| 2 | R-1.5.7     |

1. **#1** — Transform prepends "[bridged]" to message
   - **Expected:** Send "hello" from A
2. **#2** — Read in B
   - **Expected:** Value is "[bridged] hello"

### TC-1.5.7.I4 Bridge Unsubscribed Type

| # | Requirement |
|---|-------------|
| 1 | R-1.5.7     |
| 2 | R-1.5.7     |

1. **#1** — Send unsubscribed event type in world A
   - **Expected:** No error
2. **#2** — Read in world B
   - **Expected:** Absent (not bridged)

### TC-1.5.1.I1 Full Frame Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-1.5.1     |
| 2 | R-1.5.1     |

1. **#1** — Write events, swap, read, flush commands, dispatch observers
   - **Expected:** All steps complete in order
2. **#2** — Verify world state
   - **Expected:** Consistent with all operations

### TC-1.6.5.I1 Hot Reload State Preservation

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5     |

1. **#1** — Load plugin, run 1 frame, modify source, hot-reload
   - **Expected:** ECS state survives reload

### TC-1.6.5.I2 Hot Reload New Behavior

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5     |

1. **#1** — After reload with modified system logic
   - **Expected:** New system behavior is active

### TC-1.6.5.I3 Hot Reload Latency

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5a    |

1. **#1** — Reload 50-system plugin
   - **Expected:** Total cycle < 2 s

### TC-1.5.3.I1 Events Written Frame N Observed In Frame N Plus One

| # | Requirement |
|---|-------------|
| 1 | US-1.5.3    |
| 2 | US-1.5.3    |

1. **#1** — Writer system writes 1000 events in frame N
   - **Expected:** Events visible to reader system starting frame N+1 after buffer swap
2. **#2** — Reader system iterates buffer in frame N+1
   - **Expected:** All 1000 events present in insertion order; no events from frame N visible to
     frame-N readers

### TC-1.5.9.I1 Command Buffer Flush Applies All Commands Once

| # | Requirement |
|---|-------------|
| 1 | US-1.5.9    |
| 2 | US-1.5.9    |

1. **#1** — Ten parallel systems each push 100 commands into per-worker command buffers
   - **Expected:** Sync point flushes all 1000 commands in deterministic order, none dropped
2. **#2** — Repeat flush on empty buffer
   - **Expected:** No commands re-applied; world state unchanged

### TC-1.6.5.I4 Hot Reload Migration Failure

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5a    |
| 2 | R-1.6.5a    |

1. **#1** — Introduce layout change that fails migration
   - **Expected:** Error reported
2. **#2** — Verify state
   - **Expected:** Pre-reload value retained

## Benchmarks

### TC-1.5.1.B1 Event Write Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Write 100K events x 64 bytes | Time | < 1 ms | R-1.5.1a |

### TC-1.5.1.B2 Event Read Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Read 100K events, 8 parallel readers | Time | < 500 us | R-1.5.2 |

### TC-1.5.5.B1 Reactive Query Check

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 reactive queries, no changes | Total overhead | < 200 us | R-1.5.5a |

### TC-1.5.3.B1 Observer Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dispatch 1,000 observer callbacks | Time | < 2 ms | R-1.5.3 |

### TC-1.6.4.B1 Plugin Graph Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate dependency graph of 50 plugins | Time | < 1 ms | R-1.6.4 |

### TC-1.6.5.B1 Hot Reload Cycle

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full hot-reload cycle, 50 systems | Time | < 2 s | R-1.6.5a |
