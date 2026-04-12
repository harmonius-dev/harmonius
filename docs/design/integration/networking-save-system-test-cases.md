# Networking ↔ Save System Integration Test Cases

All tests below are CI-runnable and use real objects (no mocks). Cross-thread tests use real
`crossbeam-channel` instances. I/O tests use the real VFS with a tempdir; cloud tests use an
in-process fake cloud backend that implements the same non-blocking submission interface as the
platform SDKs.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.6.U1 | SaveRequestRpc rkyv round-trip | Encode, decode | Equal | IR-4.6.2 |
| TC-IR-4.6.U2 | SaveResponseRpc rkyv round-trip | Encode Success+meta | Equal | IR-4.6.2 |
| TC-IR-4.6.U3 | CheckpointPrepareRpc rkyv round-trip | Encode participants | Equal | IR-4.6.5 |
| TC-IR-4.6.U4 | CheckpointAbortRpc rkyv round-trip | Encode PeerDisconnect | Equal | IR-4.6.5 |
| TC-IR-4.6.U5 | SaveRpcBridge MPSC push/drain | Push 3, drain | FIFO order preserved | IR-4.6.2 |
| TC-IR-4.6.U6 | SaveRpcBridge multi-producer | 2 producers, 100 msgs | All 200 in FIFO | IR-4.6.2 |
| TC-IR-4.6.U7 | CloudIoRequest enum sizing | sizeof | <= 32 bytes | IR-4.6.3 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.6.1.1 | Server-only save trigger | Server trigger_save | Save completes | IR-4.6.1 |
| TC-IR-4.6.1.2 | Client cannot save directly | Client trigger_save | Rejected | IR-4.6.1 |
| TC-IR-4.6.1.3 | Serialize runs in job scope | Observe thread id | Not main thread | IR-4.6.1 |
| TC-IR-4.6.2.1 | Save RPC round-trip | Client SaveRequestRpc | Server Success | IR-4.6.2 |
| TC-IR-4.6.2.2 | Permission denied response | Unprivileged client | PermissionDenied | IR-4.6.2 |
| TC-IR-4.6.2.3 | RPC queued phase 2 to 8 | Push at P2, observe P8 | Drained at P8 | IR-4.6.2 |
| TC-IR-4.6.3.1 | Cloud upload via platform API | Fake Steam backend | Upload via Steam | IR-4.6.3 |
| TC-IR-4.6.3.2 | Cloud fallback to QUIC | No platform API | QuicFallback path | IR-4.6.3 |
| TC-IR-4.6.3.3 | Cloud sync non-blocking | Measure call time | <1 us caller time | IR-4.6.3 |
| TC-IR-4.6.4.1 | Conflict detected | Divergent hashes | SyncResult::Conflict | IR-4.6.4 |
| TC-IR-4.6.4.2 | KeepLocal resolves conflict | User KeepLocal | Local uploaded | IR-4.6.4 |
| TC-IR-4.6.4.3 | KeepRemote resolves conflict | User KeepRemote | Remote downloaded | IR-4.6.4 |
| TC-IR-4.6.5.1 | Checkpoint all clients ready | 4 clients ACK | Save triggers | IR-4.6.5 |
| TC-IR-4.6.5.2 | Checkpoint timeout | 1 client silent 5 s | AbortRpc broadcast | IR-4.6.5 |
| TC-IR-4.6.5.3 | Partial ACK then disconnect | ACK then drop | AbortRpc broadcast | IR-4.6.5 |
| TC-IR-4.6.5.4 | Post-ACK mid-save disconnect | Drop during save | In-flight aborted | IR-4.6.5 |
| TC-IR-4.6.6.1 | ConnectionId excluded | Save world w/ net | No ConnectionId out | IR-4.6.6 |
| TC-IR-4.6.6.2 | Saveable components included | Save world w/ tag | All Saveable saved | IR-4.6.6 |
| TC-IR-4.6.6.3 | ClientPredictor excluded | Save predicted ent | No ClientPredictor | IR-4.6.6 |
| TC-IR-4.6.6.4 | SnapshotInterpolator excluded | Save remote ent | No interpolator | IR-4.6.6 |
| TC-IR-4.6.6.5 | 2D world round-trip | 2D Saveable world | Identical to 3D path | IR-4.6.6 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.6.N1 | SaveRpcBridge full | Push 65 (cap 64) | try_send fails, Io err | IR-4.6.2 |
| TC-IR-4.6.N2 | CloudIoRequest ch full | Push 257 (cap 256) | CloudUnavailable err | IR-4.6.3 |
| TC-IR-4.6.N3 | Serializer panic | Inject panic in job | SaveError::Io returned | IR-4.6.1 |
| TC-IR-4.6.N4 | Disk full on write | Tempfs size limit | SaveError::Io, prior kept | IR-4.6.1 |
| TC-IR-4.6.N5 | Corrupt header on load | Flip CRC byte | Fallback to prior slot | IR-4.6.1 |
| TC-IR-4.6.N6 | Cloud timeout 30 s | Fake backend stall | Requeued next frame | IR-4.6.3 |
| TC-IR-4.6.N7 | Unknown client save | Unauthenticated | PermissionDenied | IR-4.6.2 |
| TC-IR-4.6.N8 | Malformed rkyv payload | Truncated bytes | RPC rejected | IR-4.6.2 |
| TC-IR-4.6.N9 | Save during rollback | Rollback in progress | Queued, no data loss | IR-4.6.1 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.6.1.B1 | Full save serialization (job) | < 100 ms | IR-4.6.1 |
| TC-IR-4.6.1.B2 | serialize_world main-thread cost | < 50 us (submit only) | IR-4.6.1 |
| TC-IR-4.6.2.B1 | Save RPC round-trip latency | < 200 ms (LAN) | IR-4.6.2 |
| TC-IR-4.6.2.B2 | SaveRpcBridge push+drain 1000 msgs | < 1 ms | IR-4.6.2 |
| TC-IR-4.6.3.B1 | Cloud upload 10 MB save | < 5 s (broadband) | IR-4.6.3 |
| TC-IR-4.6.3.B2 | CloudSyncAdapter::sync call cost | < 1 us | IR-4.6.3 |
| TC-IR-4.6.5.B1 | Checkpoint coord 8 clients | < 500 ms | IR-4.6.5 |
| TC-IR-4.6.5.B2 | AbortRpc broadcast 8 clients | < 50 ms | IR-4.6.5 |
| TC-IR-4.6.6.B1 | Saveable filter 100k entities | < 10 ms | IR-4.6.6 |

## Test Harness Notes

- **Real channels.** Every test constructs real `crossbeam_channel::bounded` instances with the
  production capacities (64 for `SaveRpcBridge`, 256 for `CloudIoRequest`, 16 for `SavePipeline`
  writes) and asserts backpressure behavior at the documented limits.
- **Real job system.** `serialize_world` tests spawn a real job-system `scope()` and assert the
  serialization thread id differs from the game-loop thread id.
- **Fake cloud backend.** `CloudPlatform::QuicFallback` is exercised against a real in-process QUIC
  loopback connection. The Steam / iCloud / Xbox / PlayStation / Nintendo paths are tested with a
  single `FakeCloudBackend` that implements the same non-blocking submission interface; this is not
  a mock library, it is a full fake with real state.
- **Determinism.** All timing-sensitive tests (timeouts, benchmarks) use a virtual clock so CI runs
  are reproducible.
- **CI-runnable.** All tests above run under `cargo test --workspace` with no external services;
  benchmarks run under `cargo bench` with the same harness.
