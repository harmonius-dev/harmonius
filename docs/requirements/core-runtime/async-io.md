# R-1.8 — Async I/O Requirements

## Platform I/O Backends

1. **R-1.8.1** — The engine **SHALL** define a trait-based abstraction over platform-native async
   I/O with concrete implementations for Windows (IOCP), macOS/iOS (GCD), and Linux (io_uring),
   resolved at compile time via conditional compilation and static dispatch with zero dynamic
   dispatch overhead.
   - **Rationale:** A unified I/O abstraction ensures all subsystems use platform-native async
     without falling back to Rust standard library file I/O.
   - **Verification:** Integration test per platform: submit a file read through the abstraction;
     verify completion via the native mechanism. Verify no std::fs calls via static analysis. Verify
     no vtable entries for the I/O trait.
2. **R-1.8.2** — Error codes from all three platform backends **SHALL** be mapped to a
   platform-agnostic error type so that callers handle errors uniformly.
   - **Rationale:** Subsystems must handle I/O errors without platform-specific branching.
   - **Verification:** Trigger file-not-found, permission denied, and connection refused on each
     platform; verify all map to the same agnostic error variants.

## Completion Model

1. **R-1.8.3** — The engine **SHALL** implement a completion-based (proactor) async model where the
   OS kernel notifies on I/O completion, delivering typed results carrying the operation outcome,
   bytes transferred, and a caller-supplied context token.
   - **Rationale:** All three platforms are natively completion-based, avoiding extra copies and
     retry loops of readiness-based models.
   - **Verification:** Submit 1 MB async file write; verify completion carries correct byte count
     and context token. Verify no intermediate readiness polling.
2. **R-1.8.4** — Completion notifications **SHALL** be delivered within 100 us of OS kernel
   signaling. The engine **SHALL** support at least 10,000 concurrent in-flight I/O operations
   without degrading delivery latency.
   - **Rationale:** Low completion latency is critical for frame-pacing and preventing stalls.
   - **Verification:** Submit 10,000 concurrent 4 KB reads; verify 99th percentile delivery under
     100 us.

## File I/O

1. **R-1.8.5** — The engine **SHALL** provide async file read, write, seek, and flush operations
   executing entirely through the platform backend with explicit byte offsets for positional I/O,
   never calling Rust standard library file I/O.
   - **Rationale:** Positional I/O enables concurrent access to different file regions without seek
     contention.
   - **Verification:** Write 4 MB in 1 MB chunks at explicit offsets from 4 concurrent tasks. Read
     back and verify integrity. Verify no std::fs calls via static analysis.
2. **R-1.8.6** — The engine **SHALL** support O_DIRECT / unbuffered I/O for bypassing the OS page
   cache when streaming large assets.
   - **Rationale:** Page cache bypass reduces memory pressure during high-throughput asset
     streaming.
   - **Verification:** Read a 100 MB file with unbuffered I/O; verify no increase in page cache
     usage. Compare throughput with buffered I/O.

## Network Socket I/O

1. **R-1.8.7** — The engine **SHALL** provide async TCP and UDP socket operations (accept, connect,
   send, receive, sendto/recvfrom) through the platform backend, with socket handles registered for
   async completion on creation.
   - **Rationale:** Async network I/O is the sole path for the transport layer and server
     infrastructure.
   - **Verification:** Establish TCP connection via async connect/accept; send 1 MB; verify receipt.
     Test UDP with 1,000 datagrams; verify delivery. Verify all operations complete via the platform
     mechanism.

## Audio Stream I/O

1. **R-1.8.8** — The engine **SHALL** provide low-latency async read/write operations for audio
   device buffers with deadline hints prioritizing audio above bulk file transfers, targeting sub-10
   ms round-trip latency.
   - **Rationale:** Audio underruns during asset streaming cause audible glitches that must be
     prevented.
   - **Verification:** Submit audio writes concurrently with 100 MB background reads. Verify 99th
     percentile audio latency under 10 ms. Zero underruns over 60 seconds.

## Vectored I/O

1. **R-1.8.9** — The engine **SHALL** support scatter-gather operations that read into or write from
   multiple non-contiguous buffers in a single system call.
   - **Rationale:** Vectored I/O eliminates copy overhead for composite writes and zero-copy
     protocol framing.
   - **Verification:** Write 3 non-contiguous buffers via single vectored write; read back; verify
     correct concatenation. Benchmark at least 30% reduction in system call count vs. individual
     writes.

## Priority and Scheduling

1. **R-1.8.10** — The engine **SHALL** assign priority levels (critical, normal, background) to I/O
   submissions and reorder its submission queue so critical operations are submitted ahead of
   background work.
   - **Rationale:** Deadline-aware scheduling prevents background transfers from starving
     latency-sensitive I/O during open-world streaming.
   - **Verification:** Submit 100 background reads then 1 critical read; verify critical completes
     before at least 90% of background reads.
2. **R-1.8.11** — Background I/O **SHALL NOT** be starved indefinitely; all background operations
   **SHALL** eventually complete even under sustained critical load.
   - **Rationale:** Starvation of background I/O prevents prefetch and log flushing from ever
     completing.
   - **Verification:** Sustain critical-priority load for 10 seconds; verify all queued background
     operations complete within 30 seconds.

## Cancellation

1. **R-1.8.12** — The engine **SHALL** support cancellation of in-flight I/O operations via
   cancellation tokens. The completion callback **SHALL** always fire (with either success or
   cancellation status) for deterministic resource cleanup.
   - **Rationale:** Cancelling stale loads prevents resource waste during zone transitions.
   - **Verification:** Submit 100 MB async read; cancel within 1 ms; verify callback fires with
     cancellation status. Verify no buffer or handle leaks.
2. **R-1.8.13** — Cancellation requests **SHALL** be delivered to the platform backend within 50 us.
   Every cancelled operation **SHALL** deliver a completion within 10 ms.
   - **Rationale:** Stale operations that never complete leak buffers and handles.
   - **Verification:** Submit 1,000 operations; cancel all immediately; verify every completion
     arrives within 10 ms. Verify no resource leaks.

## Buffer Management

1. **R-1.8.14** — The engine **SHALL** manage pools of pre-allocated, page-aligned I/O buffers
   registered with the platform backend, tracked via generational handles and reclaimed to the free
   list after completion. Stale handles **SHALL** be rejected with a typed error.
   - **Rationale:** Registered buffers eliminate per-operation setup costs and enable kernel page
     pinning for zero-copy I/O.
   - **Verification:** Register 64 buffers; submit 128 reads (exceeding pool); verify backpressure
     handling. Verify buffers reclaimed after completion. Benchmark at least 20% throughput
     improvement vs. unregistered buffers.
