# Events & Plugins Test Cases

Companion test cases for [events-plugins.md](events-plugins.md).

## Unit Tests

### TC-1.5.1.1 Double Buffer Swap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 3 events in frame N | Writer buffer has 3 events | R-1.5.1 |
| 2 | Advance to frame N+1, read | Reader sees 3 events | R-1.5.1 |
| 3 | Advance to frame N+2, read | Reader sees 0 events | R-1.5.1 |

### TC-1.5.1.2 Parallel Readers No Contention

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8 threads read same channel concurrently | All 8 read identical data | R-1.5.1 |
| 2 | Run under ThreadSanitizer | Zero data races | R-1.5.1 |

### TC-1.5.1.3 Flood Warning Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 50,001 events in one frame | Diagnostic warning fires | R-1.5.1a |

### TC-1.5.1.4 Throughput 100K Events

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 100,000 events of 64 bytes each | Total time < 1 ms | R-1.5.1a |

### TC-1.5.2.1 Persistent Stream Cursor

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emit 60 events across 6 frames (10/frame) | Stream has 60 events | R-1.5.2 |
| 2 | Reader at 10 Hz reads batch | Sees all 60 events | R-1.5.2 |

### TC-1.5.2.2 Cursor Independence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cursor A at position 10, Cursor B at position 30 | A sees events 10+, B sees events 30+ | R-1.5.2 |

### TC-1.5.2.3 Cursor Overflow Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cursor falls behind ring buffer wraparound | `has_overflowed()` returns true | R-1.5.2 |

### TC-1.5.3.1 Observer Fires OnAdd

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register OnAdd observer, spawn 100 entities via 4 systems | 100 callbacks received | R-1.5.3 |
| 2 | Verify callback order | Deterministic across runs | R-1.5.3 |

### TC-1.5.3.2 Observer Fires OnRemove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove component from 50 entities | 50 OnRemove callbacks fired | R-1.5.3 |

### TC-1.5.3.3 Observer Fires OnMutate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mutate component on 25 entities | 25 OnMutate callbacks fired | R-1.5.3 |

### TC-1.5.3.4 Observer Deterministic Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Repeat observer test 100 iterations | Identical callback order each time | R-1.5.3 |

### TC-1.5.4.1 Command Buffer Flush Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 systems record commands in known order | Flush applies in system execution order | R-1.5.4 |

### TC-1.5.4.2 Command Buffer Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Repeat flush 100 times with varying thread counts | Identical world state each time | R-1.5.4 |

### TC-1.5.5.1 Reactive Query Skip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reactive query on component A, 10 frames no changes | System runs 0 times | R-1.5.5 |
| 2 | Modify one A value | System runs next frame | R-1.5.5 |

### TC-1.5.5.2 Reactive Query Overhead

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 reactive queries, no changes | Total check overhead < 200 us | R-1.5.5a |

### TC-1.5.6.1 Resource Scheduler Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | System A writes via ResMut, System B reads via Res | Scheduler orders A before B | R-1.5.6 |

### TC-1.5.6.2 Resource Parallel Reads

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two read-only systems with `Res<T>` | Both run in parallel | R-1.5.6 |

### TC-1.6.1.1 Plugin Reverse Order Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register plugins C, B, A (C depends on B depends on A) | Init order: A, B, C | R-1.6.1 |

### TC-1.6.1.2 Plugin Contributions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin declares 2 systems, 1 component, 1 resource | After init, all 4 contributions exist | R-1.6.1 |

### TC-1.6.2.1 Group Disable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group of 5 plugins, disable plugin 3 | 4 active, plugin 3's systems absent | R-1.6.2 |

### TC-1.6.3.1 Missing Dependency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin X depends on absent plugin Y | Error message names "Y" | R-1.6.3 |

### TC-1.6.3.2 Conflict Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register plugin A and B that declare conflict | Conflict error with both names | R-1.6.3 |

### TC-1.6.4.1 Topological Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A -> B -> C dependency chain | Init order: A, B, C | R-1.6.4 |

### TC-1.6.4.2 Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A -> B -> A cycle | Error with cycle path [A, B, A] | R-1.6.4 |

### TC-1.6.4.3 All Errors Single Pass

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Missing dep + conflict + cycle simultaneously | All 3 errors reported in one pass | R-1.6.4a |

### TC-1.6.6.1 ABI Hash Match

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin with matching ABI hash | Load succeeds | R-1.6.6 |

### TC-1.6.6.2 ABI Hash Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin with mismatched ABI hash | Rejection error with version info | R-1.6.6 |

### TC-1.6.7.1 Capability Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register capability "physics" v1.2 | Query "physics" returns v1.2 | R-1.6.7 |
| 2 | Query "audio" (not registered) | Returns None | R-1.6.7 |

### TC-1.6.7.2 Capability Branch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | System branches on "physics" presence (registered) | Takes physics-present branch | R-1.6.7 |
| 2 | System branches on "physics" presence (absent) | Takes physics-absent branch | R-1.6.7 |

## Integration Tests

### TC-1.5.7.I1 Cross World Bridge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two worlds A and B, bridge for ChatMsg type | Bridge registered | R-1.5.7 |
| 2 | Send ChatMsg in world A | ChatMsg appears in world B | R-1.5.7 |

### TC-1.5.7.I2 Bridge Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Filter drops events where `is_private=true` | Send private + public messages | R-1.5.7 |
| 2 | Read in target world | Only public messages present | R-1.5.7 |

### TC-1.5.7.I3 Bridge Transform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transform prepends "[bridged]" to message | Send "hello" from A | R-1.5.7 |
| 2 | Read in B | Value is "[bridged] hello" | R-1.5.7 |

### TC-1.5.7.I4 Bridge Unsubscribed Type

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send unsubscribed event type in world A | No error | R-1.5.7 |
| 2 | Read in world B | Absent (not bridged) | R-1.5.7 |

### TC-1.5.1.I1 Full Frame Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write events, swap, read, flush commands, dispatch observers | All steps complete in order | R-1.5.1 |
| 2 | Verify world state | Consistent with all operations | R-1.5.1 |

### TC-1.6.5.I1 Hot Reload State Preservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin, run 1 frame, modify source, hot-reload | ECS state survives reload | R-1.6.5 |

### TC-1.6.5.I2 Hot Reload New Behavior

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | After reload with modified system logic | New system behavior is active | R-1.6.5 |

### TC-1.6.5.I3 Hot Reload Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reload 50-system plugin | Total cycle < 2 s | R-1.6.5a |

### TC-1.6.5.I4 Hot Reload Migration Failure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Introduce layout change that fails migration | Error reported | R-1.6.5a |
| 2 | Verify state | Pre-reload value retained | R-1.6.5a |

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
