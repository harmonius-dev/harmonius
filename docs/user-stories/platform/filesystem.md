# User Stories -- 14.6 Filesystem

## US-14.6.1 Read and Write Files Without Blocking Game Threads

**As a** game developer (P-15), **I want** async file open, read, and write operations that
return futures resolving on the I/O thread pool, **so that** file operations never block
the game loop or worker threads on any platform.

## US-14.6.2 Create and Delete Files Asynchronously in Batches

**As a** game developer (P-15), **I want** async file create and delete operations with
batch concurrency and recursive directory creation, **so that** bulk file operations (asset
cache cleanup, temp file management) complete quickly without blocking.

## US-14.6.3 Query File Metadata Without Blocking

**As an** engine developer (P-26), **I want** async file metadata queries (size, timestamps,
permissions, type) with batch stat for multiple paths, **so that** the asset database and
hot-reload watcher can detect changes without blocking worker threads.

## US-14.6.4 Enumerate Large Directories Incrementally

**As an** engine developer (P-26), **I want** async directory enumeration that yields entries
incrementally with configurable depth limits and glob-pattern filtering, **so that**
scanning large asset directories does not block or allocate unbounded memory.

## US-14.6.5 Detect Source File Changes for Hot Reload

**As a** game developer (P-15), **I want** directory change notifications using
platform-native APIs (ReadDirectoryChangesExW, FSEvents, inotify) with configurable
debounce intervals, **so that** the hot-reload system detects file changes within
milliseconds without polling.

## US-14.6.6 Filter Out False File Change Notifications

**As an** engine developer (P-26), **I want** BLAKE3 content hash comparison to filter out
metadata-only changes and duplicate events from editors that write via rename-into-place,
**so that** only genuine content changes trigger re-import or hot-reload.

## US-14.6.7 Resolve Paths Consistently Across Platforms

**As an** engine developer (P-26), **I want** canonical path resolution that handles Windows
drive letters, UNC paths, long-path prefixes, macOS case insensitivity, and Linux case
sensitivity, **so that** all engine subsystems use consistent keys and avoid duplicate asset
entries from path aliasing.

## US-14.6.8 Use Platform-Native Async I/O Exclusively

**As an** engine developer (P-26), **I want** all filesystem operations implemented through
IOCP on Windows, dispatch_io on macOS, and io_uring on Linux with zero Rust stdlib file
I/O anywhere in the engine, **so that** I/O performance is optimal and consistent with
project guidelines.

## US-14.6.9 Verify Async File Operations on All Platforms

**As an** engine tester (P-27), **I want** automated tests that exercise async read, write,
create, delete, stat, and enumerate operations on Windows, macOS, and Linux, verifying
correct results and no blocking on the calling thread, **so that** platform I/O regressions
are caught in CI.

## US-14.6.10 Verify File Watcher Detects All Change Types

**As an** engine tester (P-27), **I want** tests that create, modify, delete, and rename
files in watched directories and verify the correct event types are delivered with proper
debouncing, **so that** the file watcher reliably drives hot-reload on all platforms.

## US-14.6.11 Benchmark Async I/O Throughput Against Raw Disk Bandwidth

**As an** engine tester (P-27), **I want** benchmarks that measure async read and write
throughput as a percentage of raw disk bandwidth on each platform, **so that** I can verify
the I/O backend achieves at least 80% of theoretical throughput.

## US-14.6.12 Handle io_uring Fallbacks on Older Linux Kernels

**As a** DevOps engineer (P-16), **I want** fallback paths for io_uring operations that
require kernel 6.6+ (IORING_OP_GETDENTS), **so that** the engine runs correctly on older
Linux distributions used in CI and server environments.
