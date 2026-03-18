# R-14.6 — Filesystem Requirements

## Async File Operations

| ID       | Derived From                                      |
|----------|---------------------------------------------------|
| R-14.6.1 | [F-14.6.1](../../features/platform/filesystem.md) |
| R-14.6.2 | [F-14.6.2](../../features/platform/filesystem.md) |
| R-14.6.3 | [F-14.6.3](../../features/platform/filesystem.md) |
| R-14.6.4 | [F-14.6.4](../../features/platform/filesystem.md) |

1. **R-14.6.1** — The engine **SHALL** perform all file open, read, and write operations through the
   platform async I/O layer (IOCP on Windows, Grand Central Dispatch on macOS, io_uring on Linux),
   supporting sequential reads, random-access reads with explicit offsets, and buffered/unbuffered
   write modes, returning futures that resolve on the I/O thread pool without blocking game threads,
   with no Rust standard library file I/O used anywhere.
   - **Rationale:** Platform-native async I/O avoids blocking game threads during file access and
     enables concurrent reads at different offsets without seek contention, while banning stdlib
     file I/O ensures a single consistent I/O path throughout the engine.
   - **Verification:** Integration test per platform: write 4 MB in 1 MB chunks at explicit offsets
     from 4 concurrent tasks, read back at the same offsets, and verify data integrity. Verify the
     operations complete via the platform-native mechanism (IOCP/GCD/io_uring). Static analysis:
     verify no `std::fs` or `std::io::Read`/`Write` calls exist in the codebase.
2. **R-14.6.2** — The engine **SHALL** create and delete files and directories asynchronously, with
   recursive directory creation, deferred-to-close deletion for files with open handles, and batch
   deletion that issues all unlink operations concurrently.
   - **Rationale:** Async file lifecycle operations prevent blocking game threads during asset
     cleanup, cache eviction, and directory setup, while batch deletion minimizes latency for bulk
     operations.
   - **Verification:** Integration test: create a 3-level directory tree recursively and verify all
     directories exist. Delete a file with an open handle using deferred semantics and verify
     deletion completes after the handle closes. Submit a batch of 100 file deletions and verify all
     complete concurrently without blocking the calling thread.
3. **R-14.6.3** — The engine **SHALL** query file size, timestamps, permissions, and type (file,
   directory, symlink) asynchronously without blocking, with batch stat accepting multiple paths and
   issuing all queries concurrently.
   - **Rationale:** Non-blocking metadata queries prevent stalls during asset change detection and
     directory scans, while batch stat reduces round-trip overhead for multi-file queries.
   - **Verification:** Integration test: create 100 files with known sizes and timestamps. Issue a
     batch stat for all 100 paths and verify each result returns correct size, timestamp,
     permissions, and type. Verify the batch completes without blocking the calling thread by
     asserting game loop frame time does not spike.
4. **R-14.6.4** — The engine **SHALL** list directory contents asynchronously, yielding entries
   incrementally, supporting recursive traversal with configurable depth limits and glob-pattern
   filtering, and returning entry name, type, and basic metadata per entry without a separate stat
   call where the OS provides it inline.
   - **Rationale:** Incremental async enumeration prevents blocking during large directory scans
     while inline metadata avoids redundant stat calls that double I/O cost.
   - **Verification:** Integration test: create a directory tree with 1,000 files across 5 depth
     levels. Enumerate with depth limit 3 and verify only entries within that depth are returned.
     Apply a glob filter and verify only matching entries appear. Verify each entry includes name,
     type, and size without a separate stat call on platforms that provide inline metadata.

## File Watching

| ID       | Derived From                                      |
|----------|---------------------------------------------------|
| R-14.6.5 | [F-14.6.5](../../features/platform/filesystem.md) |
| R-14.6.6 | [F-14.6.6](../../features/platform/filesystem.md) |

1. **R-14.6.5** — The engine **SHALL** monitor directories for file creation, modification,
   deletion, and rename events using platform-native change notification APIs, delivering events as
   typed messages on an async channel with a configurable debounce interval, and supporting
   recursive watching of directory trees.
   - **Rationale:** Platform-native watchers provide reliable, low-latency change detection for
     hot-reload and asset pipelines, while debouncing prevents event floods during bulk operations
     like version control checkouts.
   - **Verification:** Integration test per platform: watch a directory, create a file, modify it,
     rename it, and delete it. Verify the watcher emits correctly typed events for each operation.
     Perform 100 rapid writes and verify the debounce interval coalesces them into fewer events.
     Enable recursive watching and verify events from nested subdirectories are captured.
2. **R-14.6.6** — The engine **SHALL** detect whether a watched file's content has actually changed
   by comparing BLAKE3 content hashes before and after a filesystem event, filtering out false
   positives from metadata-only changes and duplicate editor write events, triggering downstream
   actions only on genuine content changes.
   - **Rationale:** Hash-based content comparison eliminates false-positive reloads from metadata
     changes (touch, permission flip) and editor rename-into-place patterns that generate spurious
     filesystem events.
   - **Verification:** Unit test: touch a file without changing content and verify no change event
     is emitted downstream. Modify file content and verify a change event fires. Write a file via
     rename-into-place with identical content and verify no false positive. Write via
     rename-into-place with different content and verify a genuine change is detected.

## Path Management

| ID       | Derived From                                      |
|----------|---------------------------------------------------|
| R-14.6.7 | [F-14.6.7](../../features/platform/filesystem.md) |

1. **R-14.6.7** — The engine **SHALL** resolve relative paths, symlinks, and junctions to canonical
   absolute paths consistently across platforms, handling Windows drive letters, UNC paths, and
   long-path prefixes, macOS case insensitivity, and Linux case sensitivity, using canonical paths
   as keys to prevent duplicate asset entries from path aliasing.
   - **Rationale:** Canonical path resolution prevents the asset database from treating aliased
     paths (symlinks, case variants, relative vs absolute) as distinct entries, which would cause
     duplicate resource loading and wasted memory.
   - **Verification:** Unit test per platform: resolve a relative path, a symlink, and a junction to
     canonical form and verify all three produce the same canonical path. On Windows, verify UNC
     paths and `\\?\` long-path prefixes resolve correctly. On macOS, verify case-insensitive path
     variants resolve to the same canonical path. Verify two aliased paths used as asset keys map to
     the same entry.

## File Watcher Delivery

| ID       | Derived From                                      |
|----------|---------------------------------------------------|
| R-14.6.8 | [F-14.6.8](../../features/platform/filesystem.md) |

1. **R-14.6.8** — The engine **SHALL** deliver debounced, hash-verified file change events through a
   typed `FileEventStream` that exposes an `async fn next()` method returning `FileEvent` structs
   with a canonical path and a `FileEventKind` enum (Created, Modified, Deleted, Renamed), so that
   consumers can `.await` events without polling and the stream terminates when the watch is
   cancelled.
   - **Rationale:** A typed async stream provides compile-time guarantees on event structure,
     integrates with the engine's async runtime for backpressure and cancellation, and eliminates
     the need for callback registration or manual channel polling in consumer code.
   - **Verification:** Integration test: watch a directory, create a file, and call `next().await`
     on the stream. Verify the returned `FileEvent` has kind `Created` and the correct canonical
     path. Cancel the watch and verify `next().await` returns `None`. Unit test: verify the
     `FileEventKind` enum covers Created, Modified, Deleted, and Renamed variants. Verify `Renamed`
     carries the original path.

## Content Hash Management

| ID       | Derived From                                      |
|----------|---------------------------------------------------|
| R-14.6.9 | [F-14.6.9](../../features/platform/filesystem.md) |

1. **R-14.6.9** — The engine **SHALL** maintain an in-memory cache mapping canonical paths to their
   most recent BLAKE3 content hash, using the cache to avoid redundant full-file reads on repeated
   metadata-only filesystem events, and updating the cache entry when a genuine content change is
   detected.
   - **Rationale:** Without a hash cache, every filesystem event triggers a full-file read and hash
     computation even when the content has not changed. The cache reduces I/O load during bulk
     metadata operations (permission changes, timestamp updates) and editor save patterns that
     generate multiple events per write.
   - **Verification:** Unit test: insert a hash for a path, trigger a metadata-only event, and
     verify the hasher skips the event by comparing against the cached hash without reading the
     file. Modify the file content, trigger an event, and verify the cache is updated with the new
     hash. Benchmark: measure I/O operations during 1,000 rapid metadata-only events and verify no
     file reads occur after the initial hash is cached.
