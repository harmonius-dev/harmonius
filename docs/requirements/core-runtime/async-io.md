# R-1.8 — Async I/O Requirements

## Async I/O Runtime

1. **R-1.8.1** — The engine **SHALL** provide async I/O via a Tokio `current_thread` runtime,
   wrapped in an engine-specific `AsyncIo` facade that exposes file, network, and timer operations
   as `async fn` methods returning typed results.
   - **Rationale:** A single Tokio runtime unifies all async I/O behind Rust's `async`/`await`,
     eliminating the need for platform-specific backend code while keeping polling deterministic at
     the game loop's frame boundary.
   - **Verification:** Integration test per platform: submit a file read through the `AsyncIo`
     facade; verify completion when the game loop polls the runtime. Verify no `std::fs` synchronous
     calls via static analysis.
2. **R-1.8.2** — Errors from Tokio and `std::io` operations **SHALL** be mapped to a
   platform-agnostic engine error type so that callers handle errors uniformly without matching on
   OS-specific error codes.
   - **Rationale:** Subsystems must handle I/O errors without platform-specific branching.
   - **Verification:** Trigger file-not-found, permission denied, and connection refused on each
     platform; verify all map to the same engine error variants.

## Future-Based Async Model

1. **R-1.8.3** — The engine **SHALL** implement a future-based async model where I/O operations
   return futures that resolve when the underlying operation completes, carrying typed results with
   the operation outcome and bytes transferred. Tokio internally uses readiness-based polling (epoll
   on Linux, kqueue on macOS, IOCP on Windows).
   - **Rationale:** Rust's `async`/`await` with Tokio provides efficient I/O without manual
     readiness loops or callback registration.
   - **Verification:** Submit 1 MB async file write; verify the returned future resolves with
     correct byte count. Verify the game loop's poll point drives all progress.
2. **R-1.8.4** — Futures **SHALL** resolve within 100 us of the Tokio runtime being polled after the
   OS signals readiness or completion. The engine **SHALL** support at least 10,000 concurrent
   in-flight I/O operations without degrading resolution latency.
   - **Rationale:** Low future resolution latency is critical for frame-pacing and preventing
     stalls.
   - **Verification:** Submit 10,000 concurrent 4 KB reads; verify 99th percentile resolution
     latency under 100 us measured from poll invocation.

## File I/O

1. **R-1.8.5** — The engine **SHALL** provide async file read, write, seek, and flush operations via
   `tokio::fs` with explicit byte offsets for positional I/O, never calling Rust standard library
   synchronous file I/O.
   - **Rationale:** Positional I/O enables concurrent access to different file regions without seek
     contention.
   - **Verification:** Write 4 MB in 1 MB chunks at explicit offsets from 4 concurrent tasks. Read
     back and verify integrity. Verify no `std::fs` synchronous calls via static analysis.
2. **R-1.8.6** — The engine **SHALL** support O_DIRECT / unbuffered I/O for bypassing the OS page
   cache when streaming large assets, using `std::fs::OpenOptions` with platform flags to open the
   file descriptor, then wrapping it via `tokio::fs::File::from_std()`.
   - **Rationale:** Page cache bypass reduces memory pressure during high-throughput asset
     streaming.
   - **Verification:** Read a 100 MB file with unbuffered I/O; verify no increase in page cache
     usage. Compare throughput with buffered I/O.

## Network Socket I/O

1. **R-1.8.7** — The engine **SHALL** provide async TCP and UDP socket operations (accept, connect,
   send, receive, sendto/recvfrom) through `tokio::net`, with sockets registered on the Tokio
   runtime upon creation.
   - **Rationale:** Async network I/O is the sole path for the transport layer and server
     infrastructure.
   - **Verification:** Establish TCP connection via async connect/accept; send 1 MB; verify receipt.
     Test UDP with 1,000 datagrams; verify delivery. Verify all operations complete through Tokio's
     reactor.

## Audio Stream I/O

1. **R-1.8.8** — The engine **SHALL** provide low-latency async read/write operations for audio
   streaming with deadline hints prioritizing audio above bulk file transfers, targeting sub-10 ms
   round-trip latency. Disk-based audio streaming **SHALL** use `tokio::fs`. Audio device buffer I/O
   **SHALL** use platform audio APIs (WASAPI on Windows, CoreAudio on macOS/iOS, ALSA on Linux) on
   their dedicated audio threads.
   - **Rationale:** Audio underruns during asset streaming cause audible glitches that must be
     prevented. Device buffer I/O requires platform audio APIs for real-time guarantees.
   - **Verification:** Submit audio disk reads concurrently with 100 MB background reads. Verify
     99th percentile audio latency under 10 ms. Zero underruns over 60 seconds.

## Vectored I/O

1. **R-1.8.9** — The engine **SHALL** support scatter-gather operations via `tokio::io` traits
   (`AsyncRead`, `AsyncWrite`) that read into or write from multiple non-contiguous buffers in a
   single operation.
   - **Rationale:** Vectored I/O eliminates copy overhead for composite writes and zero-copy
     protocol framing.
   - **Verification:** Write 3 non-contiguous buffers via single vectored write; read back; verify
     correct concatenation. Benchmark at least 30% reduction in system call count vs. individual
     writes.

## Priority and Scheduling

1. **R-1.8.10** — The engine **SHALL** assign priority levels (critical, normal, background) to I/O
   submissions via an engine-layer priority queue that orders Tokio task spawning so critical
   operations are polled ahead of background work.
   - **Rationale:** Deadline-aware scheduling prevents background transfers from starving
     latency-sensitive I/O during open-world streaming.
   - **Verification:** Submit 100 background reads then 1 critical read; verify critical completes
     before at least 90% of background reads.
2. **R-1.8.11** — Background I/O **SHALL NOT** be starved indefinitely; all background operations
   **SHALL** eventually complete even under sustained critical load. The engine **SHALL** promote
   long-waiting background operations after a configurable timeout.
   - **Rationale:** Starvation of background I/O prevents prefetch and log flushing from ever
     completing.
   - **Verification:** Sustain critical-priority load for 10 seconds; verify all queued background
     operations complete within 30 seconds.

## Cancellation

1. **R-1.8.12** — The engine **SHALL** support cancellation of in-flight I/O operations via
   `CancellationToken` from `tokio_util::sync`. When a token is cancelled, associated futures
   **SHALL** resolve promptly (with either success or a cancellation error) for deterministic
   resource cleanup. Dropping a future **SHALL** also cancel its underlying operation.
   - **Rationale:** Cancelling stale loads prevents resource waste during zone transitions.
   - **Verification:** Submit 100 MB async read; cancel via token within 1 ms; verify the future
     resolves with a cancellation error. Verify no buffer or handle leaks.
2. **R-1.8.13** — Cancellation via token **SHALL** cause associated futures to resolve within 10 ms.
   Every cancelled operation **SHALL** release its buffers and handles upon resolution.
   - **Rationale:** Stale operations that never complete leak buffers and handles.
   - **Verification:** Submit 1,000 operations; cancel all via token immediately; verify every
     future resolves within 10 ms. Verify no resource leaks.

## Buffer Management

1. **R-1.8.14** — The engine **SHALL** manage pools of pre-allocated, page-aligned I/O buffers
   tracked via generational handles and reclaimed to the free list after future resolution. Stale
   handles **SHALL** be rejected with a typed error. Buffers **SHALL** be used with Tokio read and
   write operations to avoid per-operation allocation.
   - **Rationale:** Pooled page-aligned buffers eliminate per-operation allocation overhead and
     improve cache locality for high-throughput streaming.
   - **Verification:** Allocate 64 buffers from the pool; submit 128 reads (exceeding pool); verify
     backpressure handling. Verify buffers reclaimed after future resolution. Benchmark at least 20%
     throughput improvement vs. per-operation allocation.
