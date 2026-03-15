# 14.6 — Filesystem

## Async File Operations

### F-14.6.1 Async File Open, Read, and Write

Open, read, and write files entirely through the platform async I/O layer (F-1.8). File operations
use the core async I/O abstraction (F-1.8.1) as the underlying transport layer. The filesystem
module provides high-level file operation semantics (create, read, write, delete, enumerate) on top
of the platform I/O primitives. Supports sequential reads, random-access reads with explicit
offsets, and buffered/unbuffered write modes. All operations return futures that resolve on the I/O
thread pool without blocking game threads. No Rust stdlib file I/O is used anywhere in the engine.

- **Requirements:** R-14.6.1
- **Dependencies:** F-1.8.1, F-1.8.3
- **Platform notes:** Windows uses `CreateFileW` with `FILE_FLAG_OVERLAPPED` for IOCP integration;
  macOS uses `dispatch_io_create` / `dispatch_io_read` / `dispatch_io_write`; Linux uses `io_uring`
  `IORING_OP_READ` / `IORING_OP_WRITE`.

### F-14.6.2 Async File Create and Delete

Create and delete files and directories asynchronously. Directory creation supports recursive mkdir.
File deletion supports both immediate unlink and deferred-to-close semantics for files that may
still have open handles. Batch deletion accepts a list of paths and issues all unlink operations
concurrently.

- **Requirements:** R-14.6.2
- **Dependencies:** F-1.8.1
- **Platform notes:** Windows uses `CreateDirectoryW` / `DeleteFileW` / `RemoveDirectoryW`; macOS
  uses GCD-dispatched POSIX `mkdir` / `unlink` / `rmdir`; Linux uses io_uring `IORING_OP_MKDIRAT` /
  `IORING_OP_UNLINKAT`.

### F-14.6.3 Async File Metadata and Stat

Query file size, timestamps, permissions, and type (file, directory, symlink) without blocking.
Batch stat accepts multiple paths and issues all queries concurrently, returning results as they
complete. Used by the asset database for change detection and by the hot-reload watcher for initial
directory scans.

- **Requirements:** R-14.6.3
- **Dependencies:** F-1.8.1
- **Platform notes:** Windows uses `GetFileInformationByHandleEx`; macOS uses GCD-dispatched
  `fstatat`; Linux uses io_uring `IORING_OP_STATX`.

### F-14.6.4 Async Directory Enumeration

List directory contents asynchronously, yielding entries incrementally as they are read from the OS.
Supports recursive traversal with configurable depth limits and glob-pattern filtering. Returns
entry name, type, and basic metadata per entry without requiring a separate stat call where the OS
provides it inline.

- **Requirements:** R-14.6.4
- **Dependencies:** F-1.8.1
- **Platform notes:** Windows uses `FindFirstFileExW` / `FindNextFileW` dispatched on I/O threads;
  macOS uses GCD-dispatched `fdopendir` / `readdir`; Linux uses `io_uring` `IORING_OP_GETDENTS`
  (kernel 6.6+) with fallback to threaded `getdents64`.

## File Watching

### F-14.6.5 Directory Change Notifications

Monitor directories for file creation, modification, deletion, and rename events using
platform-native change notification APIs. Events are delivered as typed messages on an async
channel, coalesced by a configurable debounce interval to avoid flooding during bulk operations
(e.g., version control checkout). Supports recursive watching of directory trees.

- **Requirements:** R-14.6.5
- **Dependencies:** F-1.8.1, F-1.5.1
- **Platform notes:** Windows uses `ReadDirectoryChangesExW` with IOCP completion; macOS uses
  `dispatch_source_create(DISPATCH_SOURCE_TYPE_VNODE, ...)` or FSEvents for recursive watches; Linux
  uses `inotify_add_watch` with io_uring for async event reads.

### F-14.6.6 File Content Change Detection

Detect whether a watched file's content has actually changed by comparing content hashes (BLAKE3)
before and after a filesystem event. Filters out false positives from metadata-only changes (touch,
permission flip) and duplicate events from editors that write via rename-into-place. Only genuine
content changes trigger downstream re-import or hot-reload.

- **Requirements:** R-14.6.6
- **Dependencies:** F-14.6.5, F-14.6.1
- **Platform notes:** None

## Path Management

### F-14.6.7 Canonical Path Resolution

Resolve relative paths, symlinks, and junctions to canonical absolute paths consistently across
platforms. Handles Windows drive letters, UNC paths, and `\\?\` long-path prefix; macOS case
insensitivity; and Linux case sensitivity. All engine subsystems use canonical paths as keys to
avoid duplicate asset entries from path aliasing.

- **Requirements:** R-14.6.7
- **Dependencies:** None
- **Platform notes:** Windows uses `GetFinalPathNameByHandleW`; macOS uses `fcntl(F_GETPATH)` or
  `realpath`; Linux uses `realpath` or `/proc/self/fd` readlink.

## File Watcher Delivery

### F-14.6.8 Typed File Event Stream with Async Next

Deliver debounced, hash-verified file change events through a typed `FileEventStream` that exposes
an `async fn next()` method. Each event carries a `FileEvent` struct containing the canonical path
and a `FileEventKind` enum (Created, Modified, Deleted, Renamed). Consumers `.await` the next event
without polling, and the stream ends when the watch is cancelled. This replaces callback-based or
channel-based delivery with a first-class async stream integrated into the engine's async runtime.

- **Requirements:** R-14.6.8
- **Dependencies:** F-14.6.5, F-14.6.6
- **Platform notes:** None

## Content Hash Management

### F-14.6.9 Content Hash Cache for Change Detection

Maintain an in-memory cache mapping canonical paths to their most recent BLAKE3 content hash. When
the file watcher emits a Modified event, the content hasher looks up the cached hash, computes the
new hash via an async file read, and compares them. If the hashes match, the event is discarded as a
false positive. On a genuine change, the cache is updated with the new hash. The cache avoids
redundant full-file hashing on repeated metadata-only events and supports explicit cache insertion
for files whose hash is already known from import.

- **Requirements:** R-14.6.9
- **Dependencies:** F-14.6.6, F-14.6.1
- **Platform notes:** None
