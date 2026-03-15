# R-1.8 — Async I/O Requirements

## Platform I/O Backends

### R-1.8.1 Platform I/O Backend Abstraction

The engine **SHALL** define a trait-based abstraction over platform-native async I/O with concrete
implementations for Windows (IOCP), macOS/iOS (Grand Central Dispatch), and Linux (io_uring), resolved
at compile time via conditional compilation and static dispatch with zero dynamic dispatch overhead.

- **Derived from:** [F-1.8.1](../../features/core-runtime/async-io.md)
- **Rationale:** A unified I/O abstraction ensures all subsystems use platform-native async I/O without
  falling back to Rust standard library file I/O.
- **Verification:** Integration test per platform: submit a file read through the abstraction, verify
  it completes via the platform-native mechanism (IOCP/GCD/io_uring). Verify no `std::fs` calls
  exist in the codebase via static analysis. Verify the compiled binary contains no trait object
  vtable entries for the I/O trait.

## Completion Model

### R-1.8.2 Completion-Based Async Execution Model

The engine **SHALL** implement a completion-based (proactor) async model where the OS kernel notifies
the application upon I/O completion, delivering typed results carrying the operation outcome, bytes
transferred, and a caller-supplied context token.

- **Derived from:** [F-1.8.2](../../features/core-runtime/async-io.md)
- **Rationale:** All three target platforms (IOCP, GCD, io_uring) are natively completion-based,
  avoiding the extra copies and retry loops of readiness-based models.
- **Verification:** Unit test: submit an async file write of 1 MB, wait for completion, verify the
  completion result carries correct byte count and context token. Verify no intermediate readiness
  polling occurs (no `epoll`/`kqueue` calls in the call stack).

### R-1.8.2a Completion Delivery Latency

Completion notifications **SHALL** be delivered to the application within 100 microseconds of the OS
kernel signaling I/O completion. The engine **SHALL** support at least 10,000 in-flight I/O operations
concurrently without degrading completion delivery latency.

- **Derived from:** [F-1.8.2](../../features/core-runtime/async-io.md)
- **Rationale:** Low completion latency is critical for frame-pacing and preventing stalls during asset
  streaming.
- **Verification:** Benchmark: submit 10,000 concurrent 4 KB reads and measure time between OS
  completion and application callback invocation. Verify 99th percentile delivery latency is under
  100 microseconds.

## File I/O

### R-1.8.3 Async File I/O Operations

The engine **SHALL** provide async file read, write, seek, and flush operations executing entirely
through the platform I/O backend with explicit byte offsets for positional I/O, without ever calling
Rust standard library file I/O functions.

- **Derived from:** [F-1.8.3](../../features/core-runtime/async-io.md)
- **Rationale:** Platform-native async file I/O avoids blocking worker threads and enables concurrent
  access to different file regions without seek contention.
- **Verification:** Integration test: write 4 MB to a file in 1 MB chunks at explicit offsets from 4
  concurrent tasks. Read back at the same offsets and verify data integrity. Verify no `std::fs::read`,
  `std::fs::write`, or `std::io::Read`/`Write` trait calls via static analysis.

## Network Socket I/O

### R-1.8.4 Async Network Socket I/O

The engine **SHALL** provide async TCP and UDP socket operations (accept, connect, send, receive,
sendto/recvfrom) through the platform I/O backend, with socket handles registered for async completion
on creation.

- **Derived from:** [F-1.8.4](../../features/core-runtime/async-io.md)
- **Rationale:** Async network I/O is the sole path for the transport layer, server infrastructure,
  and telemetry systems.
- **Verification:** Integration test: establish a TCP connection between two local sockets via async
  connect/accept. Send 1 MB of data and verify receipt. Test UDP sendto/recvfrom with 1,000 datagrams
  and verify delivery count. Verify all operations complete via the platform completion mechanism.

## Audio Stream I/O

### R-1.8.5 Async Audio Stream I/O

The engine **SHALL** provide low-latency async read/write operations for audio device buffers with
deadline hints that prioritize audio I/O above bulk file transfers, targeting sub-10 ms round-trip
latency on all platforms.

- **Derived from:** [F-1.8.5](../../features/core-runtime/async-io.md)
- **Rationale:** Audio buffer underruns during intensive asset streaming cause audible glitches that
  must be prevented by I/O prioritization.
- **Verification:** Benchmark: submit audio writes concurrently with 100 MB of background file reads.
  Measure audio completion latency and verify it remains under 10 ms for 99th percentile. Verify zero
  buffer underruns over 60 seconds of continuous playback.

## Vectored I/O

### R-1.8.6 Scatter-Gather and Vectored I/O

The engine **SHALL** support scatter-gather operations that read into or write from multiple
non-contiguous memory buffers in a single system call, reducing per-operation overhead and kernel
transitions.

- **Derived from:** [F-1.8.6](../../features/core-runtime/async-io.md)
- **Rationale:** Vectored I/O eliminates copy overhead when composing multi-part writes (header +
  metadata + payload) and enables zero-copy network protocol framing.
- **Verification:** Unit test: write 3 non-contiguous buffers (header, metadata, payload) via a single
  vectored write. Read back the file and verify the contents are concatenated in order. Benchmark:
  compare vectored write throughput to individual writes and verify at least 30% reduction in system
  call count.

## Priority and Scheduling

### R-1.8.7 I/O Priority and Deadline Scheduling

The engine **SHALL** assign priority levels (critical, normal, background) to I/O submissions and
reorder its internal submission queue so that critical operations (audio, frame-critical assets)
are submitted ahead of background work (prefetch, telemetry, log flushing).

- **Derived from:** [F-1.8.7](../../features/core-runtime/async-io.md)
- **Rationale:** Deadline-aware scheduling prevents background bulk transfers from starving
  latency-sensitive I/O during open-world streaming.
- **Verification:** Benchmark: submit 100 background-priority file reads followed by 1 critical-priority
  read. Measure completion order and verify the critical read completes before at least 90% of the
  background reads.

## Cancellation

### R-1.8.8 Cooperative I/O Cancellation

The engine **SHALL** support cancellation of in-flight I/O operations via cancellation tokens, where the
completion callback fires with a cancellation status to enable deterministic resource cleanup.

- **Derived from:** [F-1.8.8](../../features/core-runtime/async-io.md)
- **Rationale:** Cancelling stale asset loads and network requests prevents resource waste during zone
  transitions and camera movement.
- **Verification:** Unit test: submit a large async file read (100 MB), cancel it via the cancellation
  token within 1 ms. Verify the completion callback fires with cancellation status. Verify no
  resource leaks (buffer returned to pool, file handle state is valid).

### R-1.8.8a Cancellation Latency and Completeness

Cancellation requests **SHALL** be delivered to the platform backend within 50 microseconds of the
token being cancelled. The engine **SHALL** guarantee that every cancelled operation eventually delivers
a completion (either success if completed before cancellation, or cancellation status), preventing
resource leaks from orphaned operations.

- **Derived from:** [F-1.8.8](../../features/core-runtime/async-io.md)
- **Rationale:** Stale operations that are never completed leak buffers and file handles; cancellation
  must be reliable to prevent resource exhaustion during zone transitions.
- **Verification:** Submit 1,000 operations and cancel all immediately. Verify every operation delivers
  a completion within 10 ms. Verify no buffer or handle leaks via allocator and handle tracking.

## Buffer Management

### R-1.8.9 Registered Buffer Pools and Zero-Copy Transfers

The engine **SHALL** manage pools of pre-allocated, page-aligned I/O buffers registered with the
platform backend, tracked via generational handles and reclaimed to the free list after completion
processing, enabling zero-copy kernel transfers.

- **Derived from:** [F-1.8.9](../../features/core-runtime/async-io.md)
- **Rationale:** Registered buffers eliminate per-operation buffer setup costs and enable kernel page
  pinning for zero-copy I/O at high throughput.
- **Verification:** Integration test: register a pool of 64 buffers. Submit 128 read operations
  (exceeding pool size). Verify the pool handles backpressure correctly (queuing or blocking). Verify
  buffers are reclaimed after completion. Benchmark: compare throughput with registered vs unregistered
  buffers and verify at least 20% improvement for sequential 4 KB reads.
