# R-14.6 — Filesystem Requirements

## Async File Operations

### R-14.6.1 Async File Open, Read, and Write

The engine **SHALL** perform all file open, read, and write operations through the platform async
I/O layer (IOCP on Windows, Grand Central Dispatch on macOS, io_uring on Linux), supporting
sequential reads, random-access reads with explicit offsets, and buffered/unbuffered write modes,
returning futures that resolve on the I/O thread pool without blocking game threads, with no Rust
standard library file I/O used anywhere.

- **Derived from:** [F-14.6.1](../../features/platform/filesystem.md)
- **Rationale:** Platform-native async I/O avoids blocking game threads during file access and
  enables concurrent reads at different offsets without seek contention, while banning stdlib file
  I/O ensures a single consistent I/O path throughout the engine.
- **Verification:** Integration test per platform: write 4 MB in 1 MB chunks at explicit offsets
  from 4 concurrent tasks, read back at the same offsets, and verify data integrity. Verify the
  operations complete via the platform-native mechanism (IOCP/GCD/io_uring). Static analysis:
  verify no `std::fs` or `std::io::Read`/`Write` calls exist in the codebase.

### R-14.6.2 Async File Create and Delete

The engine **SHALL** create and delete files and directories asynchronously, with recursive
directory creation, deferred-to-close deletion for files with open handles, and batch deletion that
issues all unlink operations concurrently.

- **Derived from:** [F-14.6.2](../../features/platform/filesystem.md)
- **Rationale:** Async file lifecycle operations prevent blocking game threads during asset cleanup,
  cache eviction, and directory setup, while batch deletion minimizes latency for bulk operations.
- **Verification:** Integration test: create a 3-level directory tree recursively and verify all
  directories exist. Delete a file with an open handle using deferred semantics and verify deletion
  completes after the handle closes. Submit a batch of 100 file deletions and verify all complete
  concurrently without blocking the calling thread.

### R-14.6.3 Async File Metadata and Stat

The engine **SHALL** query file size, timestamps, permissions, and type (file, directory, symlink)
asynchronously without blocking, with batch stat accepting multiple paths and issuing all queries
concurrently.

- **Derived from:** [F-14.6.3](../../features/platform/filesystem.md)
- **Rationale:** Non-blocking metadata queries prevent stalls during asset change detection and
  directory scans, while batch stat reduces round-trip overhead for multi-file queries.
- **Verification:** Integration test: create 100 files with known sizes and timestamps. Issue a
  batch stat for all 100 paths and verify each result returns correct size, timestamp, permissions,
  and type. Verify the batch completes without blocking the calling thread by asserting game loop
  frame time does not spike.

### R-14.6.4 Async Directory Enumeration

The engine **SHALL** list directory contents asynchronously, yielding entries incrementally,
supporting recursive traversal with configurable depth limits and glob-pattern filtering, and
returning entry name, type, and basic metadata per entry without a separate stat call where the OS
provides it inline.

- **Derived from:** [F-14.6.4](../../features/platform/filesystem.md)
- **Rationale:** Incremental async enumeration prevents blocking during large directory scans while
  inline metadata avoids redundant stat calls that double I/O cost.
- **Verification:** Integration test: create a directory tree with 1,000 files across 5 depth
  levels. Enumerate with depth limit 3 and verify only entries within that depth are returned.
  Apply a glob filter and verify only matching entries appear. Verify each entry includes name,
  type, and size without a separate stat call on platforms that provide inline metadata.

## File Watching

### R-14.6.5 Directory Change Notifications

The engine **SHALL** monitor directories for file creation, modification, deletion, and rename
events using platform-native change notification APIs, delivering events as typed messages on an
async channel with a configurable debounce interval, and supporting recursive watching of directory
trees.

- **Derived from:** [F-14.6.5](../../features/platform/filesystem.md)
- **Rationale:** Platform-native watchers provide reliable, low-latency change detection for
  hot-reload and asset pipelines, while debouncing prevents event floods during bulk operations
  like version control checkouts.
- **Verification:** Integration test per platform: watch a directory, create a file, modify it,
  rename it, and delete it. Verify the watcher emits correctly typed events for each operation.
  Perform 100 rapid writes and verify the debounce interval coalesces them into fewer events.
  Enable recursive watching and verify events from nested subdirectories are captured.

### R-14.6.6 File Content Change Detection

The engine **SHALL** detect whether a watched file's content has actually changed by comparing
BLAKE3 content hashes before and after a filesystem event, filtering out false positives from
metadata-only changes and duplicate editor write events, triggering downstream actions only on
genuine content changes.

- **Derived from:** [F-14.6.6](../../features/platform/filesystem.md)
- **Rationale:** Hash-based content comparison eliminates false-positive reloads from metadata
  changes (touch, permission flip) and editor rename-into-place patterns that generate spurious
  filesystem events.
- **Verification:** Unit test: touch a file without changing content and verify no change event is
  emitted downstream. Modify file content and verify a change event fires. Write a file via
  rename-into-place with identical content and verify no false positive. Write via rename-into-place
  with different content and verify a genuine change is detected.

## Path Management

### R-14.6.7 Canonical Path Resolution

The engine **SHALL** resolve relative paths, symlinks, and junctions to canonical absolute paths
consistently across platforms, handling Windows drive letters, UNC paths, and long-path prefixes,
macOS case insensitivity, and Linux case sensitivity, using canonical paths as keys to prevent
duplicate asset entries from path aliasing.

- **Derived from:** [F-14.6.7](../../features/platform/filesystem.md)
- **Rationale:** Canonical path resolution prevents the asset database from treating aliased paths
  (symlinks, case variants, relative vs absolute) as distinct entries, which would cause duplicate
  resource loading and wasted memory.
- **Verification:** Unit test per platform: resolve a relative path, a symlink, and a junction to
  canonical form and verify all three produce the same canonical path. On Windows, verify UNC paths
  and `\\?\` long-path prefixes resolve correctly. On macOS, verify case-insensitive path variants
  resolve to the same canonical path. Verify two aliased paths used as asset keys map to the same
  entry.
