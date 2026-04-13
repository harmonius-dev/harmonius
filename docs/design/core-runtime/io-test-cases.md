# I/O Request Protocol Test Cases

Companion test cases for [io.md](io.md).

## Unit Tests

### TC-1.8.1.1 IoRequest ReadFile Construction

| # | Requirement |
|---|-------------|
| 1 | F-1.8.1     |
| 2 | F-1.8.1     |

1. **#1** — Build `IoRequest::ReadFile { id, path: VPath("asset://mesh.bin"), buffer }`
   - **Expected:** Enum discriminant matches `IoRequestKind::Read`
2. **#2** — Request carries caller-owned `IoRequestId`
   - **Expected:** `req.id() == expected_id`

### TC-1.8.1.2 IoRequest SendPacket Construction

| # | Requirement |
|---|-------------|
| 1 | F-1.8.1     |

1. **#1** — Build `IoRequest::SendPacket { id, socket, buffer }` with a 1500-byte payload
   - **Expected:** `buffer.len == 1500`; enum discriminant matches `IoRequestKind::Socket`

### TC-1.8.1.3 IoRequest SwapChainPresent Construction

| # | Requirement |
|---|-------------|
| 1 | F-1.8.1     |

1. **#1** — Build `IoRequest::SwapChainPresent { id, swapchain, image_index: 0 }`
   - **Expected:** Carries both `swapchain` handle and `image_index`

### TC-1.8.1.4 IoRequest CancelRequest Carries Target

| # | Requirement |
|---|-------------|
| 1 | F-1.8.4     |

1. **#1** — Build `IoRequest::CancelRequest { target: IoRequestId(42) }`
   - **Expected:** `target == IoRequestId(42)`; no buffer required

### TC-1.8.2.1 IoResponse ReadOk Parsing

| # | Requirement |
|---|-------------|
| 1 | F-1.8.2     |
| 2 | F-1.8.2     |

1. **#1** — Match on `IoResponse::ReadOk { id, bytes_read: 2048, buffer }`
   - **Expected:** Caller destructures `bytes_read == 2048`
2. **#2** — Returned buffer has `len == 2048`
   - **Expected:** `buffer.len == 2048`

### TC-1.8.2.2 IoResponse Failed Carries IoError

| # | Requirement |
|---|-------------|
| 1 | F-1.8.2     |

1. **#1** — Match on `IoResponse::Failed { id, error: IoError::NotFound { .. } }`
   - **Expected:** Caller destructures `IoError::NotFound` variant

### TC-1.8.2.3 IoResponse Cancelled Only Carries Id

| # | Requirement |
|---|-------------|
| 1 | F-1.8.4     |

1. **#1** — Build `IoResponse::Cancelled { id }` and match
   - **Expected:** Response carries only the original `IoRequestId`; no buffer returned

### TC-1.8.3.1 VirtualFileSystem Resolve Asset Mount

| # | Requirement |
|---|-------------|
| 1 | F-1.8.3     |

1. **#1** — Mount `asset://` -> `LooseFiles("/game/assets")`, resolve `asset://mesh.bin`
   - **Expected:** `Some(PathBuf("/game/assets/mesh.bin"))`

### TC-1.8.3.2 VirtualFileSystem Unknown Prefix

| # | Requirement |
|---|-------------|
| 1 | F-1.8.3     |

1. **#1** — Resolve `unknown://foo` with no matching mount
   - **Expected:** `None`

### TC-1.8.3.3 VirtualFileSystem Mount Shadowing

| # | Requirement |
|---|-------------|
| 1 | F-1.8.3     |

1. **#1** — Mount `asset://` as PackedArchive, then mount `asset://` as LooseFiles; resolve
   - **Expected:** Returns the later LooseFiles mount (editor overlay semantics)

### TC-1.8.6.1 IoBufferPool Acquire Release

| # | Requirement |
|---|-------------|
| 1 | F-1.8.6     |
| 2 | F-1.8.6     |

1. **#1** — `acquire(4096)`
   - **Expected:** Returns `IoBuffer` with `capacity >= 4096`
2. **#2** — Release; re-acquire same size
   - **Expected:** Same slot reused; zero additional allocation

### TC-1.8.6.2 IoBufferPool Bucketed Sizes

| # | Requirement |
|---|-------------|
| 1 | F-1.8.6     |

1. **#1** — Acquire sizes 64, 512, 4096, 65536
   - **Expected:** Each routes to a distinct bucket; capacity rounded up

### TC-1.8.5.1 Dispatcher In-Flight Ticket Bookkeeping

| # | Requirement |
|---|-------------|
| 1 | F-1.8.5     |

1. **#1** — Submit 3 requests, do not drain completions
   - **Expected:** `in_flight.len() == 3`; tickets sorted by `IoRequestId`

### TC-1.8.4.1 Cancel Moves Ticket To Cancelled State

| # | Requirement |
|---|-------------|
| 1 | F-1.8.4     |
| 2 | F-1.8.4     |

1. **#1** — Submit read, then submit cancel for same id, drain completions
   - **Expected:** Exactly one `IoResponse::Cancelled { id }` emitted
2. **#2** — `in_flight` no longer contains the cancelled id
   - **Expected:** `in_flight.get(id).is_none()`

## Integration Tests

### TC-1.8.5.2 Main Thread Drain Single Completion

| # | Requirement |
|---|-------------|
| 1 | F-1.8.5     |

1. **#1** — Subsystem sends `ReadFile`, dispatcher polls, loopback backend completes immediately
   - **Expected:** After `poll_completions()`, subsystem receives `IoResponse::ReadOk`

### TC-1.8.5.3 Main Thread Drain Multiple Subsystems

| # | Requirement |
|---|-------------|
| 1 | F-1.8.5     |
| 2 | F-1.8.5     |

1. **#1** — Asset, save, and networking each submit one request in parallel
   - **Expected:** All three responses arrive within one `poll_completions()` call
2. **#2** — Response order matches deterministic per-subsystem queue order
   - **Expected:** Responses for a single subsystem preserve submission order

### TC-1.8.5.4 Main Thread Drain Backpressure

| # | Requirement |
|---|-------------|
| 1 | F-1.8.5     |

1. **#1** — Flood channel with 10,000 requests in one frame, single `poll_completions()` call
   - **Expected:** Dispatcher processes queue without dropping requests; no panic

### TC-1.8.3.4 Asset Loader End-to-End

| # | Requirement |
|---|-------------|
| 1 | F-1.8.3     |
| 2 | F-1.8.5     |

1. **#1** — Asset pipeline reads `asset://mesh.bin` via dispatcher, parses header
   - **Expected:** Header magic bytes match expected asset format
2. **#2** — On completion, buffer is released back to pool
   - **Expected:** Pool reports zero leaked buffers at end of frame

### TC-1.8.1.5 Networking Send Receive Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-1.8.1     |
| 2 | F-1.8.2     |

1. **#1** — Open loopback socket, send 1 KiB packet, receive 1 KiB packet
   - **Expected:** Sent and received bytes match byte-for-byte
2. **#2** — Dispatcher emits `OpenSocketOk`, `SendPacketOk`, `RecvPacketOk` in order
   - **Expected:** Three distinct responses, all referencing the submitted `IoRequestId`s

### TC-1.8.1.6 Save System Write File

| # | Requirement |
|---|-------------|
| 1 | F-1.8.1     |

1. **#1** — Save system writes 16 KiB via `WriteFile` to `save://slot0.bin`
   - **Expected:** `WriteOk` response with `bytes_written == 16384`

### TC-1.8.4.2 Cancel Before Completion

| # | Requirement |
|---|-------------|
| 1 | F-1.8.4     |

1. **#1** — Submit `ReadFile` to slow backend, cancel before completion, drain
   - **Expected:** `IoResponse::Cancelled`; no `ReadOk` ever arrives

## Benchmarks

### TC-1.8.5.5 10K Small-File Reads Under 100 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.8.5a    |

1. **#1** — 10,000 `ReadFile` requests of 1 KiB each through loopback backend
   - **Expected:** All 10,000 complete within 100 ms on reference workstation

### TC-1.8.5.6 Request Submission Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit ReadFile via MPSC | Latency | < 500 ns | R-1.8.5a |
| 2 | Poll completions, no work | Overhead | < 200 ns | R-1.8.5a |

### TC-1.8.6.3 IoBufferPool Acquire Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Acquire 4 KiB buffer, warm pool | Latency | < 100 ns | R-1.8.6a |
| 2 | Acquire 1 MiB buffer, warm pool | Latency | < 500 ns | R-1.8.6a |

### TC-1.8.3.5 VFS Resolve Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Resolve with 16 mounts | Latency | < 200 ns | R-1.8.3a |
| 2 | Resolve with 64 mounts | Latency | < 500 ns | R-1.8.3a |
