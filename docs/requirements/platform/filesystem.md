# R-14.6 — Filesystem Requirements

## Async File Operations

1. **R-14.6.1** — The engine **SHALL** perform all file open, read, and write operations through the
   platform async I/O layer (IOCP on Windows, GCD on macOS, io_uring on Linux), supporting
   sequential reads, random-access reads with explicit offsets, and buffered/unbuffered write modes,
   returning futures that resolve on the I/O thread pool without blocking game threads. No Rust
   standard library file I/O **SHALL** be used anywhere.
   - **Rationale:** Platform-native async I/O avoids blocking game threads and enables concurrent
     reads at different offsets. Banning stdlib I/O ensures a single consistent I/O path.
   - **Verification:** Integration test per platform: write 4 MB in 1 MB chunks at explicit offsets
     from 4 concurrent tasks, read back, verify integrity. Static analysis: verify no `std::fs` or
     `std::io::Read`/`Write` calls exist.

2. **R-14.6.2** — The engine **SHALL** create and delete files and directories asynchronously, with
   recursive directory creation, deferred-to-close deletion for files with open handles, and batch
   deletion issuing all unlinks concurrently.
   - **Rationale:** Async lifecycle operations prevent blocking during cache eviction and directory
     setup. Batch deletion minimizes latency.
   - **Verification:** Integration test: create a 3-level directory tree recursively; verify all
     exist. Delete a file with an open handle using deferred semantics; verify deletion after handle
     close. Batch-delete 100 files; verify all complete concurrently.

3. **R-14.6.3** — The engine **SHALL** query file size, timestamps, permissions, and type
   asynchronously, with batch stat accepting multiple paths concurrently.
   - **Rationale:** Non-blocking metadata queries prevent stalls during change detection. Batch stat
     reduces round-trip overhead.
   - **Verification:** Integration test: create 100 files with known sizes. Batch stat all; verify
     correct results. Assert no game loop frame time spike.

4. **R-14.6.4** — The engine **SHALL** list directory contents asynchronously, yielding entries
   incrementally, with configurable depth limits, glob-pattern filtering, and inline metadata per
   entry where the OS provides it.
   - **Rationale:** Incremental enumeration prevents blocking during large scans. Inline metadata
     avoids redundant stat calls.
   - **Verification:** Integration test: create 1,000 files across 5 depths. Enumerate with depth
     limit 3; verify correct entries. Apply glob filter; verify only matches appear.

## File Watching

5. **R-14.6.5** — The engine **SHALL** monitor directories for file creation, modification,
   deletion, and rename events using platform-native APIs, delivering typed messages on an async
   channel with a configurable debounce interval, and supporting recursive watching.
   - **Rationale:** Platform-native watchers provide reliable, low-latency change detection.
     Debouncing prevents floods during bulk operations.
   - **Verification:** Integration test per platform: watch a directory, create/modify/rename/delete
     a file; verify correct event types. Perform 100 rapid writes; verify debounce coalesces events.

6. **R-14.6.6** — The engine **SHALL** detect whether a watched file's content has changed by
   comparing BLAKE3 hashes before and after a filesystem event, filtering out metadata-only changes
   and duplicate editor write events.
   - **Rationale:** Hash comparison eliminates false-positive reloads from metadata changes and
     rename-into-place patterns.
   - **Verification:** Unit test: touch a file without changing content; verify no downstream event.
     Modify content; verify change event fires. Write via rename-into-place with identical content;
     verify no false positive.

## Path Management

7. **R-14.6.7** — The engine **SHALL** resolve relative paths, symlinks, and junctions to canonical
   absolute paths consistently across platforms, handling Windows drive letters, UNC paths,
   long-path prefixes, macOS case insensitivity, and Linux case sensitivity, using canonical paths
   as keys to prevent duplicate asset entries.
   - **Rationale:** Canonical path resolution prevents the asset database from treating aliased
     paths as distinct entries.
   - **Verification:** Unit test per platform: resolve a relative path, symlink, and junction to
     canonical form; verify all produce the same path. Verify UNC and long-path on Windows. Verify
     case-insensitive resolution on macOS.

## File Watcher Delivery

8. **R-14.6.8** — The engine **SHALL** deliver debounced, hash-verified file change events through a
   typed `FileEventStream` with `async fn next()` returning `FileEvent` structs with canonical path
   and `FileEventKind` enum (Created, Modified, Deleted, Renamed). The stream **SHALL** terminate
   when the watch is cancelled.
   - **Rationale:** A typed async stream provides compile-time guarantees, integrates with the async
     runtime for backpressure and cancellation, and eliminates callback registration.
   - **Verification:** Integration test: watch a directory, create a file, call `next().await`;
     verify `Created` kind and correct path. Cancel watch; verify `next()` returns `None`.

## Content Hash Management

9. **R-14.6.9** — The engine **SHALL** maintain an in-memory cache mapping canonical paths to their
   most recent BLAKE3 content hash, using the cache to skip redundant full-file reads on repeated
   metadata-only events, and updating the entry on genuine content change. The cache **SHALL**
   support explicit insertion for files whose hash is known from import.
   - **Rationale:** Without a hash cache, every event triggers a full-file read and hash. The cache
     reduces I/O during bulk metadata operations.
   - **Verification:** Unit test: insert a hash, trigger metadata-only event, verify no file read.
     Modify content, trigger event, verify cache updated. Benchmark: 1,000 rapid metadata events;
     verify no file reads after initial hash.

## Performance

10. **R-14.6.10** — Async file read and write throughput **SHALL** achieve at least 80 % of raw disk
    bandwidth on each platform.
    - **Rationale:** The async I/O abstraction must not introduce significant overhead compared to
      raw platform I/O.
    - **Verification:** Benchmark per platform: measure sequential read/write throughput for a 1 GB
      file and compare against raw disk throughput from a platform-native tool.

## Linux Kernel Compatibility

11. **R-14.6.11** — The engine **SHALL** use io_uring exclusively for all async I/O on Linux, with
    threaded POSIX fallback for operations requiring kernel 6.6+ (e.g., `IORING_OP_GETDENTS`). The
    minimum Linux kernel **SHALL** be 5.1.
    - **Rationale:** io_uring provides the highest performance I/O model on Linux. Threaded fallback
      ensures compatibility with older kernels without requiring an epoll fallback path.
    - **Verification:** Integration test on kernel 5.1: enumerate a directory and verify the
      threaded fallback is used. On kernel 6.6+, verify `IORING_OP_GETDENTS` is used.
