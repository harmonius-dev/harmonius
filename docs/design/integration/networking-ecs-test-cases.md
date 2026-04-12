# Networking ↔ ECS Integration Test Cases

All tests in this file are **CI-runnable**: they require no GPU, no real network, and no external
servers. Transport is exercised via an in-process loopback fake that emulates packet loss, reorder,
and RTT. 2D and 2.5D networking are out of scope (see design Scope section).

## Integration Tests

### Positive Cases

| ID | Name | Req |
|----|------|-----|
| TC-IR-4.4.1.1 | Changed component detected | IR-4.4.1 |
| TC-IR-4.4.1.2 | Unchanged component skipped | IR-4.4.1 |
| TC-IR-4.4.2.1 | Delta packet contains only diff | IR-4.4.2 |
| TC-IR-4.4.2.2 | Baseline advances on ACK | IR-4.4.2 |
| TC-IR-4.4.2.3 | Dense-array baseline indexed | IR-4.4.2 |
| TC-IR-4.4.2.4 | rkyv archive round-trip | IR-4.4.2 |
| TC-IR-4.4.3.1 | Nearby entity is relevant | IR-4.4.3 |
| TC-IR-4.4.3.2 | Distant entity filtered out | IR-4.4.3 |
| TC-IR-4.4.3.3 | Grid used not BVH | IR-4.4.3 |
| TC-IR-4.4.4.1 | Entity spawn replicated | IR-4.4.4 |
| TC-IR-4.4.4.2 | Entity despawn tombstoned | IR-4.4.4 |
| TC-IR-4.4.4.3 | Tombstone TTL expires | IR-4.4.4 |
| TC-IR-4.4.5.1 | Snapshot stores N ticks | IR-4.4.5 |
| TC-IR-4.4.5.2 | Snapshot restore correct | IR-4.4.5 |
| TC-IR-4.4.5.3 | CoW shared range resolves | IR-4.4.5 |
| TC-IR-4.4.5.4 | mmap arena path works | IR-4.4.5 |
| TC-IR-4.4.6.1 | Entity becomes dormant | IR-4.4.6 |
| TC-IR-4.4.6.2 | Dormant entity wakes | IR-4.4.6 |
| TC-IR-4.4.7.1 | Authority transfer completes | IR-4.4.7 |
| TC-IR-4.4.7.2 | Transfer input buffered on worker | IR-4.4.7 |
| TC-IR-4.4.7.3 | Stale epoch inputs dropped | IR-4.4.7 |
| TC-IR-4.4.8.1 | Reconciliation replays inputs | IR-4.4.8 |
| TC-IR-4.4.8.2 | Correct prediction skips replay | IR-4.4.8 |
| TC-IR-4.4.8.3 | Replay uses FixedUpdate | IR-4.4.8 |
| TC-IR-4.4.8.4 | Custom filter fn ptr table | IR-4.4.1 |
| TC-IR-4.4.8.5 | Debug toggle runtime flip | IR-4.4.1 |

1. **TC-IR-4.4.1.1** -- Input: modify a replicated component on one entity. Expected: `Changed<T>`
   query returns that entity.
2. **TC-IR-4.4.1.2** -- Input: no modification. Expected: `Changed<T>` query is empty.
3. **TC-IR-4.4.2.1** -- Input: change 1 of 5 fields on a component. Expected: delta packet contains
   exactly 1 field run.
4. **TC-IR-4.4.2.2** -- Input: client ACKs tick 100. Expected: `DeltaTracker` baseline tick equals
   100.
5. **TC-IR-4.4.2.3** -- Input: mutate chunk slot 7. Expected: resulting `DeltaRun.offset` matches
   the byte range for slot 7 in the dense baseline array.
6. **TC-IR-4.4.2.4** -- Input: archive and deserialize a `DeltaPayload` via rkyv. Expected: all
   fields equal the original.
7. **TC-IR-4.4.3.1** -- Input: entity within client AOI radius. Expected: `RelevantSet` contains the
   entity.
8. **TC-IR-4.4.3.2** -- Input: entity beyond client AOI radius. Expected: `RelevantSet` excludes the
   entity.
9. **TC-IR-4.4.3.3** -- Input: construct an `InterestManager`. Expected: the `grid` field is a
   `NetworkGrid`, not a shared BVH handle.
10. **TC-IR-4.4.4.1** -- Input: server spawns an entity. Expected: client receives a full component
    snapshot.
11. **TC-IR-4.4.4.2** -- Input: server despawns an entity. Expected: client receives a tombstone
    marker.
12. **TC-IR-4.4.4.3** -- Input: advance time by `2 * max_rtt`. Expected: client removes the entity.
13. **TC-IR-4.4.5.1** -- Input: run 128 ticks. Expected: `SnapshotBuffer` holds 128 snapshots.
14. **TC-IR-4.4.5.2** -- Input: restore tick 50. Expected: world state matches the recorded tick 50
    state.
15. **TC-IR-4.4.5.3** -- Input: tick with one unchanged chunk. Expected: that chunk is stored as
    `ChunkRange::Shared`, not duplicated.
16. **TC-IR-4.4.5.4** -- Input: enable `MmapArena`. Expected: snapshot byte writes land in the
    mapped region and survive buffer recycle.
17. **TC-IR-4.4.6.1** -- Input: no changes for `threshold_ticks`. Expected: `DormancyManager`
    reports the entity dormant.
18. **TC-IR-4.4.6.2** -- Input: component change on a dormant entity. Expected: dormancy cleared and
    replication resumes.
19. **TC-IR-4.4.7.1** -- Input: run the three-phase protocol to completion. Expected: new authority
    owns the entity at the new epoch.
20. **TC-IR-4.4.7.2** -- Input: inject input during transfer. Expected: the `InputBuffer` that
    receives it lives on the worker thread (thread-id assertion).
21. **TC-IR-4.4.7.3** -- Input: deliver an input tagged with the old epoch after an epoch bump.
    Expected: discarded.
22. **TC-IR-4.4.8.1** -- Input: client/server mismatch at tick T. Expected: after replay, client
    state matches server state.
23. **TC-IR-4.4.8.2** -- Input: client/server match at tick T. Expected: no replay, input buffer
    trimmed to entries after T.
24. **TC-IR-4.4.8.3** -- Input: replay 10 inputs. Expected: each runs under the `Phase::FixedUpdate`
    pipeline (fixed-step instrumentation confirms).
25. **TC-IR-4.4.8.4** -- Input: evaluate `ReplicationCondition::Custom(id)`. Expected: the codegen'd
    function pointer at index `id` is called.
26. **TC-IR-4.4.8.5** -- Input: flip `debug.replication.overlay` at runtime. Expected: overlay
    enabled on the next `ReplicationSystem::tick`.

### Negative Cases

| ID | Name | Req |
|----|------|-----|
| TC-IR-4.4.2.N1 | ACK for unknown client | IR-4.4.2 |
| TC-IR-4.4.2.N2 | Delta from evicted baseline | IR-4.4.2 |
| TC-IR-4.4.3.N1 | Client position unknown | IR-4.4.3 |
| TC-IR-4.4.4.N1 | Tombstone for unknown entity | IR-4.4.4 |
| TC-IR-4.4.4.N2 | Spawn then immediate despawn | IR-4.4.4 |
| TC-IR-4.4.5.N1 | get_at missing tick | IR-4.4.5 |
| TC-IR-4.4.5.N2 | Shared range source evicted | IR-4.4.5 |
| TC-IR-4.4.6.N1 | Wake unknown entity | IR-4.4.6 |
| TC-IR-4.4.7.N1 | Transfer timeout aborts | IR-4.4.7 |
| TC-IR-4.4.7.N2 | SnapshotAcked lost | IR-4.4.7 |
| TC-IR-4.4.8.N1 | Reconcile with missing snapshot | IR-4.4.8 |
| TC-IR-4.4.8.N2 | MPSC ack_rx overflow | IR-4.4.2 |
| TC-IR-4.4.8.N3 | Packet loss retransmit | IR-4.4.1 |

1. **TC-IR-4.4.2.N1** -- Input: ACK carrying an invalid `ConnectionId`. Expected: ignored, no panic,
   no baseline mutation.
2. **TC-IR-4.4.2.N2** -- Input: client baseline pruned before delta compute. Expected:
   `DeltaTracker` triggers a full resync.
3. **TC-IR-4.4.3.N1** -- Input: client whose position is not yet known (pre-spawn). Expected:
   `InterestManager::evaluate` returns an empty set; replication waits one tick.
4. **TC-IR-4.4.4.N1** -- Input: tombstone for an unknown entity ID. Expected: discarded silently.
5. **TC-IR-4.4.4.N2** -- Input: spawn and despawn the same entity in one tick. Expected: a single
   tombstone is emitted, no spawn packet.
6. **TC-IR-4.4.5.N1** -- Input: `get_at(tick)` for a tick older than the ring buffer. Expected:
   returns `None`.
7. **TC-IR-4.4.5.N2** -- Input: force eviction of a snapshot referenced by a `ChunkRange::Shared`
   entry. Expected: full resync is triggered for affected clients.
8. **TC-IR-4.4.6.N1** -- Input: `DormancyManager::wake` for an unknown entity. Expected: no-op, no
   panic.
9. **TC-IR-4.4.7.N1** -- Input: no ACK within `transfer_timeout_ticks`. Expected: transfer aborts,
   original authority retained.
10. **TC-IR-4.4.7.N2** -- Input: drop the `SnapshotAcked` packet. Expected: abort after the timeout
    elapses.
11. **TC-IR-4.4.8.N1** -- Input: reconcile when the target snapshot has been evicted. Expected:
    accept server state, no replay, visual pop acknowledged.
12. **TC-IR-4.4.8.N2** -- Input: flood 512 ACKs into an `ack_rx` channel with capacity 256.
    Expected: overflow drop counter increments; no deadlock.
13. **TC-IR-4.4.8.N3** -- Input: drop 1 of 3 consecutive delta packets. Expected: next-tick delta
    contains the state lost to the dropped packet.

## Benchmarks

All benchmarks run under `cargo bench`, reproducible in CI, single-threaded unless noted.

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.4.1.B1 | Change detection 10k entities | < 0.2 ms | IR-4.4.1 |
| TC-IR-4.4.2.B1 | Delta compression 1000 entities | < 0.3 ms | IR-4.4.2 |
| TC-IR-4.4.2.B2 | Dense-array XOR diff 10k bytes | < 0.05 ms | IR-4.4.2 |
| TC-IR-4.4.3.B1 | Interest mgmt 64 clients | < 0.5 ms | IR-4.4.3 |
| TC-IR-4.4.4.B1 | Spawn replication 1000 entities | < 0.4 ms | IR-4.4.4 |
| TC-IR-4.4.4.B2 | Despawn tombstone 1000 entities | < 0.2 ms | IR-4.4.4 |
| TC-IR-4.4.5.B1 | Snapshot capture 10k entities | < 0.5 ms | IR-4.4.5 |
| TC-IR-4.4.5.B2 | Snapshot restore 10k entities | < 0.8 ms | IR-4.4.5 |
| TC-IR-4.4.6.B1 | Dormancy transition 10k entities | < 0.1 ms | IR-4.4.6 |
| TC-IR-4.4.6.B2 | Dormant skip loop 10k entities | < 0.02 ms | IR-4.4.6 |
| TC-IR-4.4.7.B1 | Authority transfer 100 entities | < 1.0 ms | IR-4.4.7 |
| TC-IR-4.4.7.B2 | Epoch bump tag scan | < 0.1 ms | IR-4.4.7 |
| TC-IR-4.4.8.B1 | Reconciliation replay 10 inputs | < 1 ms | IR-4.4.8 |
