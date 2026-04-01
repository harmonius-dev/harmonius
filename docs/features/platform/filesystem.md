# 14.6 — Filesystem

## Async File Operations

| ID       | Feature                          | Requirements |
|----------|----------------------------------|--------------|
| F-14.6.1 | Async File Open, Read, and Write | R-14.6.1     |
| F-14.6.2 | Async File Create and Delete     | R-14.6.2     |
| F-14.6.3 | Async File Metadata and Stat     | R-14.6.3     |
| F-14.6.4 | Async Directory Enumeration      | R-14.6.4     |

1. **F-14.6.1** — Open, read, and write files entirely through the platform async I/O layer (F-1.8).
   File operations use the core async I/O abstraction (F-1.8.1) as the underlying transport layer.
   The filesystem module provides high-level file operation semantics (create, read, write, delete,
   enumerate) on top of the platform I/O primitives. Supports sequential reads, random-access reads
   with explicit offsets, and buffered/unbuffered write modes. All operations return futures that
   resolve on the I/O thread pool without blocking game threads. No Rust stdlib file I/O is used
   anywhere in the engine.
   - **Deps:** F-1.8.1, F-1.8.3
   - **Platform:** All file I/O goes through tokio::fs. Tokio handles platform dispatch internally
     (IOCP on Windows, kqueue on macOS, epoll on Linux).
2. **F-14.6.2** — Create and delete files and directories asynchronously. Directory creation
   supports recursive mkdir. File deletion supports both immediate unlink and deferred-to-close
   semantics for files that may still have open handles. Batch deletion accepts a list of paths and
   issues all unlink operations concurrently.
   - **Deps:** F-1.8.1
   - **Platform:** All directory and file deletion goes through tokio::fs. Tokio handles platform
     dispatch internally.
3. **F-14.6.3** — Query file size, timestamps, permissions, and type (file, directory, symlink)
   without blocking. Batch stat accepts multiple paths and issues all queries concurrently,
   returning results as they complete. Used by the asset database for change detection and by the
   hot-reload watcher for initial directory scans.
   - **Deps:** F-1.8.1
   - **Platform:** File metadata queries go through tokio::fs. Tokio handles platform dispatch
     internally.
4. **F-14.6.4** — List directory contents asynchronously, yielding entries incrementally as they are
   read from the OS. Supports recursive traversal with configurable depth limits and glob-pattern
   filtering. Returns entry name, type, and basic metadata per entry without requiring a separate
   stat call where the OS provides it inline.
   - **Deps:** F-1.8.1
   - **Platform:** Directory listing goes through tokio::fs. Tokio handles platform dispatch
     internally.

## File Watching

| ID       | Feature                        | Requirements |
|----------|--------------------------------|--------------|
| F-14.6.5 | Directory Change Notifications | R-14.6.5     |
| F-14.6.6 | File Content Change Detection  | R-14.6.6     |

1. **F-14.6.5** — Monitor directories for file creation, modification, deletion, and rename events
   using platform-native change notification APIs. Events are delivered as typed messages on an
   async channel, coalesced by a configurable debounce interval to avoid flooding during bulk
   operations (e.g., version control checkout). Supports recursive watching of directory trees.
   - **Deps:** F-1.8.1, F-1.5.1
   - **Platform:** Windows uses `ReadDirectoryChangesExW` with IOCP completion; macOS uses
     `dispatch_source_create(DISPATCH_SOURCE_TYPE_VNODE, ...)` or FSEvents for recursive watches;
     Linux uses `inotify_add_watch` with Tokio for async event reads.
2. **F-14.6.6** — Detect whether a watched file's content has actually changed by comparing content
   hashes (BLAKE3) before and after a filesystem event. Filters out false positives from
   metadata-only changes (touch, permission flip) and duplicate events from editors that write via
   rename-into-place. Only genuine content changes trigger downstream re-import or hot-reload.
   - **Deps:** F-14.6.5, F-14.6.1

## Path Management

| ID       | Feature                   | Requirements |
|----------|---------------------------|--------------|
| F-14.6.7 | Canonical Path Resolution | R-14.6.7     |

1. **F-14.6.7** — Resolve relative paths, symlinks, and junctions to canonical absolute paths
   consistently across platforms. Handles Windows drive letters, UNC paths, and `\\?\` long-path
   prefix; macOS case insensitivity; and Linux case sensitivity. All engine subsystems use canonical
   paths as keys to avoid duplicate asset entries from path aliasing.
   - **Platform:** Windows uses `GetFinalPathNameByHandleW`; macOS uses `fcntl(F_GETPATH)` or
     `realpath`; Linux uses `realpath` or `/proc/self/fd` readlink.

## File Watcher Delivery

| ID       | Feature                                 | Requirements |
|----------|-----------------------------------------|--------------|
| F-14.6.8 | Typed File Event Stream with Async Next | R-14.6.8     |

1. **F-14.6.8** — Deliver debounced, hash-verified file change events through a typed
   `FileEventStream` that exposes an `async fn next()` method. Each event carries a `FileEvent`
   struct containing the canonical path and a `FileEventKind` enum (Created, Modified, Deleted,
   Renamed). Consumers `.await` the next event without polling, and the stream ends when the watch
   is cancelled. This replaces callback-based or channel-based delivery with a first-class async
   stream integrated into the engine's async runtime.
   - **Deps:** F-14.6.5, F-14.6.6

## Content Hash Management

| ID       | Feature                                 | Requirements |
|----------|-----------------------------------------|--------------|
| F-14.6.9 | Content Hash Cache for Change Detection | R-14.6.9     |

1. **F-14.6.9** — Maintain an in-memory cache mapping canonical paths to their most recent BLAKE3
   content hash. When the file watcher emits a Modified event, the content hasher looks up the
   cached hash, computes the new hash via an async file read, and compares them. If the hashes
   match, the event is discarded as a false positive. On a genuine change, the cache is updated with
   the new hash. The cache avoids redundant full-file hashing on repeated metadata-only events and
   supports explicit cache insertion for files whose hash is already known from import.
   - **Deps:** F-14.6.6, F-14.6.1
