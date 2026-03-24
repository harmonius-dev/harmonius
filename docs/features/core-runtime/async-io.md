# 1.8 — Async I/O

## Platform I/O Backends

| ID      | Feature                          | Requirements |
|---------|----------------------------------|--------------|
| F-1.8.1 | Platform I/O Backend Abstraction | R-1.8.1      |

1. **F-1.8.1** — Define a trait-based abstraction over platform-native async I/O primitives, with
   concrete implementations for Windows (IOCP), macOS/iOS (Grand Central Dispatch), and Linux
   (io_uring). Backend selection is resolved at compile time via conditional compilation and static
   dispatch, producing zero-overhead platform calls with no trait objects or dynamic dispatch. Every
   I/O operation in the engine routes through this abstraction, ensuring no subsystem bypasses the
   unified I/O layer.
   - **Platform:** Windows backend wraps `CreateIoCompletionPort` and `GetQueuedCompletionStatusEx`
     via C COM wrappers through bindgen. macOS backend wraps `dispatch_io_create`,
     `dispatch_io_read`, and `dispatch_io_write` via Swift @_cdecl C ABI wrappers. Linux
     backend wraps liburing (`io_uring_prep_*`, `io_uring_submit`, `io_uring_wait_cqe_nr`) via
     bindgen.

## Completion Model

| ID      | Feature                                | Requirements |
|---------|----------------------------------------|--------------|
| F-1.8.2 | Completion-Based Async Execution Model | R-1.8.2      |

1. **F-1.8.2** — Implement a completion-based (proactor) async model where the application submits
   I/O requests and receives notifications when the OS kernel has finished the operation. All three
   platform backends (IOCP, GCD, io_uring) are natively completion-based, avoiding the epoll/kqueue
   readiness-based model that requires additional copies and retry loops. Completions are delivered
   as typed results carrying the operation outcome, bytes transferred, and a caller-supplied context
   token for correlation.
   - **Deps:** F-1.8.1
   - **Platform:** Mobile: max 32 in-flight I/O operations to conserve kernel resources. Switch: max
     64 in-flight operations. Desktop: max 256 in-flight. High-end PC: max 1024 in-flight for
     parallel asset streaming.

## File I/O

| ID      | Feature                   | Requirements |
|---------|---------------------------|--------------|
| F-1.8.3 | Async File I/O Operations | R-1.8.3      |

1. **F-1.8.3** — Provide async file read, write, seek, and flush operations that execute entirely
   through the platform I/O backend without ever calling Rust standard library file I/O. Reads and
   writes accept explicit byte offsets for positional I/O, enabling concurrent access to different
   regions of the same file without seek contention. All asset loading, save game persistence, log
   writing, and configuration reads use these operations as their sole file access path.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Windows uses `ReadFile`/`WriteFile` with `OVERLAPPED` structures. macOS uses
     `dispatch_io_read`/`dispatch_io_write` with explicit offsets. Linux uses `io_uring_prep_read`/
     `io_uring_prep_write` with offset fields. All backends support O_DIRECT / unbuffered I/O for
     bypass of OS page cache when streaming large assets.

## Network Socket I/O

| ID      | Feature                  | Requirements |
|---------|--------------------------|--------------|
| F-1.8.4 | Async Network Socket I/O | R-1.8.4      |

1. **F-1.8.4** — Provide async TCP and UDP socket operations — accept, connect, send, receive, and
   sendto/recvfrom — through the platform I/O backend. Socket handles are registered with the
   backend's completion mechanism on creation so that all subsequent operations complete
   asynchronously. This is the sole network I/O path for the transport layer, MMO server
   infrastructure, voice chat, and telemetry systems.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Windows registers sockets with IOCP via `CreateIoCompletionPort` and uses
     `WSASend`/ `WSARecv` with overlapped structures. macOS uses `dispatch_source_create` with
     `DISPATCH_SOURCE_TYPE_READ` and `DISPATCH_SOURCE_TYPE_WRITE` on socket file descriptors. Linux
     uses `io_uring_prep_send`/ `io_uring_prep_recv` and `io_uring_prep_accept` for server sockets.

## Audio Stream I/O

| ID      | Feature                | Requirements |
|---------|------------------------|--------------|
| F-1.8.5 | Async Audio Stream I/O | R-1.8.5      |

1. **F-1.8.5** — Provide low-latency async read and write operations for audio device buffers,
   enabling the audio engine to stream decoded samples to output devices and capture microphone
   input without blocking the audio thread. Audio I/O submissions carry deadline hints so the
   platform backend can prioritize them above bulk file transfers, preventing buffer underruns
   during intensive asset streaming.
   - **Deps:** F-1.8.1, F-1.8.2, F-1.8.7
   - **Platform:** Windows uses WASAPI in event-driven mode with IOCP notification for buffer
     completion. macOS uses Core Audio's `AudioUnit` render callbacks fed by `dispatch_io` ring
     buffers. Linux uses ALSA with io_uring-backed period notifications. All platforms target
     sub-10ms round-trip latency.

## Vectored I/O

| ID      | Feature                         | Requirements |
|---------|---------------------------------|--------------|
| F-1.8.6 | Scatter-Gather and Vectored I/O | R-1.8.6      |

1. **F-1.8.6** — Support scatter-gather operations that read into or write from multiple
   non-contiguous memory buffers in a single system call, reducing per-operation overhead and kernel
   transitions. Vectored I/O enables the asset pipeline to assemble composite writes (header +
   metadata + payload) without copying into a contiguous staging buffer, and enables the network
   layer to prepend protocol headers to payload buffers at zero copy cost.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Windows uses `WSASend`/`WSARecv` with `WSABUF` arrays and `ReadFileScatter`/
     `WriteFileGather`. macOS uses `dispatch_io_read`/`dispatch_io_write` with `dispatch_data`
     composites that reference multiple buffers. Linux uses
     `io_uring_prep_readv`/`io_uring_prep_writev` with `iovec` arrays.

## Priority and Scheduling

| ID      | Feature                              | Requirements |
|---------|--------------------------------------|--------------|
| F-1.8.7 | I/O Priority and Deadline Scheduling | R-1.8.7      |

1. **F-1.8.7** — Assign priority levels (critical, normal, background) and optional deadline hints
   to I/O submissions. The backend reorders its internal submission queue so that critical
   operations (audio buffers, frame-critical asset loads) are submitted ahead of background work
   (prefetch, telemetry uploads, log flushing). Deadline-aware scheduling prevents background bulk
   transfers from starving latency-sensitive I/O during open-world streaming where hundreds of asset
   requests may be in flight simultaneously.
   - **Deps:** F-1.8.1
   - **Platform:** Windows uses `SetFileIoOverlappedRange` and thread-pool priority for IOCP
     workers. macOS uses GCD QoS classes (`QOS_CLASS_USER_INTERACTIVE` for audio,
     `QOS_CLASS_BACKGROUND` for prefetch). Linux uses `io_uring_prep_*` with `IOSQE_IO_LINK`
     chaining and `ioprio_set` for per-request priority.

## Cancellation

| ID      | Feature                      | Requirements |
|---------|------------------------------|--------------|
| F-1.8.8 | Cooperative I/O Cancellation | R-1.8.8      |

1. **F-1.8.8** — Support cancellation of in-flight I/O operations via cancellation tokens that are
   passed at submission time. Cancelling a token requests the platform backend to abort the
   associated operation; the completion callback still fires with a cancellation status so callers
   can clean up resources deterministically. This enables the asset streaming system to cancel
   pending loads for chunks the player has moved away from, and the networking layer to abort stale
   requests during zone transitions.
   - **Deps:** F-1.8.1, F-1.8.2
   - **Platform:** Windows uses `CancelIoEx` targeting specific overlapped structures. macOS uses
     `dispatch_io_close` with `DISPATCH_IO_STOP` or `dispatch_source_cancel`. Linux uses
     `io_uring_prep_cancel` targeting the user_data token of the original submission.

## Buffer Management

| ID      | Feature                                         | Requirements |
|---------|-------------------------------------------------|--------------|
| F-1.8.9 | Registered Buffer Pools and Zero-Copy Transfers | R-1.8.9      |

1. **F-1.8.9** — Manage pools of pre-allocated, page-aligned I/O buffers that are registered with
   the platform backend to eliminate per-operation buffer setup costs. Registered buffers enable the
   kernel to pin physical pages and skip copy-on-read/write, reducing CPU overhead and memory
   bandwidth consumption for high-throughput asset streaming. The pool tracks buffer lifetimes via
   generational handles, reclaiming buffers to the free list once their associated completion has
   been processed.
   - **Deps:** F-1.8.1, F-1.7.3 (Typed Pool Allocator), F-1.7.4 (Generational Index Handles)
   - **Platform:** Windows uses `SetFileIoOverlappedRange` for buffer registration. macOS uses
     `dispatch_data_create` with `DISPATCH_DATA_DESTRUCTOR_FREE` for zero-copy dispatch data
     wrappers. Linux uses `io_uring_register_buffers` to pin a buffer table in the kernel,
     referenced by index in subsequent `io_uring_prep_read_fixed`/`io_uring_prep_write_fixed` calls.
