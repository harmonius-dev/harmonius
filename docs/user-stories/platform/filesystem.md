# User Stories -- 14.6 Filesystem

| ID         | Persona                 |
|------------|-------------------------|
| US-14.6.1  | game developer (P-15)   |
| US-14.6.2  | game developer (P-15)   |
| US-14.6.3  | game developer (P-15)   |
| US-14.6.4  | game developer (P-15)   |
| US-14.6.5  | game developer (P-15)   |
| US-14.6.6  | game developer (P-15)   |
| US-14.6.7  | engine developer (P-26) |
| US-14.6.8  | engine developer (P-26) |
| US-14.6.9  | engine developer (P-26) |
| US-14.6.10 | engine developer (P-26) |
| US-14.6.11 | engine developer (P-26) |
| US-14.6.12 | engine developer (P-26) |
| US-14.6.13 | DevOps engineer (P-16)  |
| US-14.6.14 | engine tester (P-27)    |
| US-14.6.15 | engine tester (P-27)    |
| US-14.6.16 | engine tester (P-27)    |
| US-14.6.17 | engine tester (P-27)    |

## Async File Operations

1. **US-14.6.1** — **As a** game developer (P-15), **I want** async file open, read, and write
   operations that return futures resolving on the I/O thread pool, **so that** file operations
   never block the game loop or worker threads.
2. **US-14.6.2** — **As a** game developer (P-15), **I want** async file create and delete with
   batch concurrency and recursive directory creation, **so that** bulk file operations complete
   quickly without blocking.
3. **US-14.6.3** — **As a** game developer (P-15), **I want** async file metadata queries (size,
   timestamps, permissions, type) with batch stat for multiple paths, **so that** asset change
   detection does not block worker threads.
4. **US-14.6.4** — **As a** game developer (P-15), **I want** async directory enumeration that
   yields entries incrementally with configurable depth limits and glob filtering, **so that**
   scanning large asset directories does not block or allocate unbounded memory.

## File Watching

5. **US-14.6.5** — **As a** game developer (P-15), **I want** directory change notifications using
   platform-native APIs with configurable debounce, **so that** the hot-reload system detects file
   changes within milliseconds without polling.
6. **US-14.6.6** — **As a** game developer (P-15), **I want** file change events delivered through a
   typed `FileEventStream` with `async fn next()`, **so that** I can consume events using `.await`
   without polling or registering callbacks.

## Path and Hash Management

7. **US-14.6.7** — **As an** engine developer (P-26), **I want** canonical path resolution that
   handles Windows drive letters, UNC paths, macOS case insensitivity, and Linux case sensitivity,
   **so that** all subsystems use consistent keys and avoid duplicate asset entries.
8. **US-14.6.8** — **As an** engine developer (P-26), **I want** BLAKE3 content hash comparison to
   filter out metadata-only changes and duplicate editor write events, **so that** only genuine
   content changes trigger re-import or hot-reload.
9. **US-14.6.9** — **As an** engine developer (P-26), **I want** the content hasher to maintain an
   in-memory cache of BLAKE3 hashes keyed by canonical path, **so that** repeated metadata-only
   events do not trigger redundant full-file reads.
10. **US-14.6.10** — **As an** engine developer (P-26), **I want** to insert a known BLAKE3 hash
    into the cache after an asset import, **so that** the next filesystem event can compare without
    re-reading.

## Platform I/O Backends

11. **US-14.6.11** — **As an** engine developer (P-26), **I want** all filesystem operations
    implemented through IOCP on Windows, dispatch_io on macOS, and io_uring on Linux with zero Rust
    stdlib file I/O, **so that** I/O performance is optimal and consistent with project guidelines.
12. **US-14.6.12** — **As an** engine developer (P-26), **I want** threaded POSIX fallback paths for
    io_uring operations that require kernel 6.6+, **so that** the engine runs on any Linux kernel
    5.1+ without requiring an epoll fallback.

## DevOps

13. **US-14.6.13** — **As a** DevOps engineer (P-16), **I want** the minimum Linux kernel
    requirement to be 5.1 with io_uring used exclusively for all async I/O, **so that** CI
    environments can be standardized on a single async I/O backend.

## Engine Tester -- Validation

14. **US-14.6.14** — **As an** engine tester (P-27), **I want** tests that exercise async read,
    write, create, delete, stat, and enumerate operations on all platforms, **so that** platform I/O
    regressions are caught in CI.
15. **US-14.6.15** — **As an** engine tester (P-27), **I want** tests that create, modify, delete,
    and rename files in watched directories and verify correct event types are delivered with
    debouncing, **so that** the file watcher reliably drives hot-reload.
16. **US-14.6.16** — **As an** engine tester (P-27), **I want** benchmarks that measure async read
    and write throughput as a percentage of raw disk bandwidth, **so that** the I/O backend achieves
    at least 80 % of theoretical throughput.
17. **US-14.6.17** — **As an** engine tester (P-27), **I want** tests that cancel a file watch and
    verify `FileEventStream::next().await` returns `None`, **so that** stream termination semantics
    are validated.
