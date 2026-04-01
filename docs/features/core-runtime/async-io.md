# 1.8 — Async I/O

## Tokio-Based Async I/O

| ID      | Feature                            |
|---------|------------------------------------|
| F-1.8.1 | Tokio-Based Async I/O Abstraction |

1. **F-1.8.1** — Define a thin `AsyncIo` facade over Tokio `current_thread` runtime that provides
   the engine's sole async I/O surface. All file, network, and streaming I/O operations route
   through this facade, which delegates to `tokio::fs` for file operations and `tokio::net` for
   socket operations. Tokio handles platform I/O internally via mio, selecting the optimal OS
   mechanism at compile time. No subsystem bypasses the facade or calls stdlib blocking I/O
   directly.
   - **Platform:** Tokio uses IOCP on Windows, kqueue on macOS, epoll on Linux internally. The
     engine does not interact with these mechanisms directly.

## Async Future Model

| ID      | Feature                       |
|---------|-------------------------------|
| F-1.8.2 | Async Future-Based I/O Model |

1. **F-1.8.2** — Use Tokio's async/await model where futures resolve when I/O readiness or
   completion is signalled by the OS. The `current_thread` runtime drives a single-threaded event
   loop that the game loop polls at each frame boundary. I/O futures yield typed results carrying
   the operation outcome, bytes transferred, and a caller-supplied context token for correlation.
   Tokio uses a readiness-based model (epoll/kqueue) on Linux/macOS and a completion-based model
   (IOCP) on Windows; the engine treats both uniformly through the async/await abstraction.
   - **Deps:** F-1.8.1
   - **Platform:** Tokio uses IOCP on Windows, kqueue on macOS, epoll on Linux internally. The
     runtime is polled cooperatively by the game loop, not run on a background thread.

## File I/O

| ID      | Feature                  |
|---------|---------------------------|
| F-1.8.3 | Async File I/O Operations|

1. **F-1.8.3** — Provide async file read, write, seek, and flush operations via `tokio::fs`, which
   delegates blocking file system calls to Tokio's internal thread pool. No Rust standard library
   file I/O is called directly by engine subsystems. All asset loading, save game persistence, log
   writing, and configuration reads use `tokio::fs` as their sole file access path. Reads and writes
   accept explicit byte offsets for positional I/O, enabling concurrent access to different regions
   of the same file without seek contention.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** O_DIRECT / unbuffered I/O for bypass of OS page cache when streaming large assets
     is achieved by opening files with platform flags via `std::fs::OpenOptions`, then converting to
     a Tokio file handle with `tokio::fs::File::from_std()`.

## Network Socket I/O

| ID      | Feature                 |
|---------|--------------------------|
| F-1.8.4 | Async Network Socket I/O|

1. **F-1.8.4** — Provide async TCP and UDP socket operations — accept, connect, send, receive, and
   sendto/recvfrom — through `tokio::net`. `TcpListener`, `TcpStream`, and `UdpSocket` are the sole
   network I/O types for the transport layer, server infrastructure, voice chat, and telemetry
   systems. Sockets are automatically registered with the Tokio runtime's I/O driver on creation so
   that all subsequent operations complete asynchronously via async/await.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Tokio uses IOCP on Windows, kqueue on macOS, epoll on Linux internally. The
     engine uses `tokio::net` types exclusively and does not interact with OS socket APIs directly.

## Audio Stream I/O

| ID      | Feature               |
|---------|------------------------|
| F-1.8.5 | Async Audio Stream I/O|

1. **F-1.8.5** — Audio device buffer I/O uses platform-specific audio APIs (WASAPI on Windows, Core
   Audio on macOS, ALSA on Linux) for low-latency sample streaming to output devices and microphone
   capture. These audio device APIs operate outside Tokio because they require dedicated real-time
   threads and platform callbacks. Async I/O for streaming audio file data from disk (decoding,
   prefetching, ring buffer fills) uses `tokio::fs` through the `AsyncIo` facade. Audio file I/O
   submissions carry priority hints so the engine scheduling layer can prioritize them above bulk
   transfers, preventing buffer underruns during intensive asset streaming.
   - **Deps:** F-1.8.1, F-1.8.3, F-1.8.7
   - **Platform:** Device I/O: WASAPI event-driven mode on Windows, Core Audio `AudioUnit` render
     callbacks on macOS, ALSA period callbacks on Linux. File I/O: `tokio::fs` for all audio file
     streaming. All platforms target sub-10ms round-trip device latency.

## Vectored I/O

| ID      | Feature                        |
|---------|---------------------------------|
| F-1.8.6 | Scatter-Gather and Vectored I/O|

1. **F-1.8.6** — Support scatter-gather operations that read into or write from multiple
   non-contiguous memory buffers in a single call, reducing per-operation overhead and kernel
   transitions. Vectored I/O is available via `tokio::io::AsyncReadExt::read_vectored` and
   `tokio::io::AsyncWriteExt::write_vectored`, which delegate to the OS vectored I/O primitives. The
   asset pipeline uses vectored writes to assemble composite output (header + metadata + payload)
   without copying into a contiguous staging buffer. The network layer uses vectored sends to
   prepend protocol headers to payload buffers at zero copy cost.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Tokio delegates vectored I/O to OS primitives: `readv`/`writev` on Linux and
     macOS, `WSASend`/`WSARecv` with `WSABUF` arrays on Windows. The engine uses the Tokio async
     traits and does not call these OS APIs directly.

## Priority and Scheduling

| ID      | Feature                             |
|---------|--------------------------------------|
| F-1.8.7 | I/O Priority and Deadline Scheduling|

1. **F-1.8.7** — Assign priority levels (critical, normal, background) and optional deadline hints
   to I/O submissions. Tokio does not expose OS-level I/O priority, so the engine implements an
   application-level priority queue in its scheduling layer above Tokio. The scheduler reorders
   pending I/O task spawns so that critical operations (audio file reads, frame-critical asset
   loads) are spawned ahead of background work (prefetch, telemetry uploads, log flushing).
   Deadline-aware scheduling prevents background bulk transfers from starving latency-sensitive I/O
   during open-world streaming where hundreds of asset requests may be in flight.
   - **Deps:** F-1.8.1
   - **Platform:** The priority queue is engine-managed and platform-independent. OS-level I/O
     priority (`ioprio_set` on Linux, QoS classes on macOS) is not used because Tokio does not
     expose per-request priority to the OS I/O layer.

## Cancellation

| ID      | Feature                     |
|---------|------------------------------|
| F-1.8.8 | Cooperative I/O Cancellation|

1. **F-1.8.8** — Support cancellation of in-flight I/O operations via
   `tokio_util::sync::CancellationToken`. Tokens are passed at task spawn time and checked
   cooperatively within async task bodies using `tokio::select!` to race the I/O future against the
   cancellation signal. Dropping a future also cancels its associated I/O work. The completion path
   returns a cancellation status so callers can clean up resources deterministically. This enables
   the asset streaming system to cancel pending loads for chunks the player has moved away from, and
   the networking layer to abort stale requests during zone transitions.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Cancellation is handled entirely in user-space by Tokio's cooperative task model.
     No OS-level cancellation APIs are called directly.

## Buffer Management

| ID      | Feature                            |
|---------|------------------------------------|
| F-1.8.9 | Engine-Managed I/O Buffer Pools   |

1. **F-1.8.9** — Manage pools of pre-allocated, page-aligned I/O buffers within the engine's memory
   subsystem for use with Tokio I/O operations. Buffers are drawn from the pool before issuing async
   reads and returned after the completion future resolves, avoiding per-operation allocation
   overhead and reducing memory fragmentation during high-throughput asset streaming. The pool
   tracks buffer lifetimes via generational handles, reclaiming buffers to the free list once their
   associated I/O future has completed. Unlike kernel-registered buffer schemes, these buffers are
   managed entirely in user-space without OS buffer pinning.
   - **Deps:** F-1.8.1, F-1.7.3 (Typed Pool Allocator), F-1.7.4 (Generational Index Handles)
   - **Platform:** Buffer pools are engine-managed and platform-independent. No kernel buffer
     registration is performed. Buffers are passed to `tokio::fs` and `tokio::net` operations as
     standard byte slices.
