# Platform OS Integration Test Cases

Companion test cases for [os-integration.md](os-integration.md).

## Unit Tests

### TC-14.2.1.1 Clipboard Text Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |
| 2 | R-14.2.1    |
| 3 | R-14.2.1    |

1. **#1** — Write UTF-8 string "Hello 世界 🌍" to clipboard
   - **Expected:** `read_text()` returns identical string
2. **#2** — Write empty string "" to clipboard
   - **Expected:** `read_text()` returns `Ok("")`
3. **#3** — Write 1 MB UTF-8 string to clipboard
   - **Expected:** `read_text()` returns identical string

### TC-14.2.1.2 Clipboard Image Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |
| 2 | R-14.2.1    |

1. **#1** — Write 256x256 RGBA image to clipboard
   - **Expected:** `read_image()` returns matching pixel data within tolerance
2. **#2** — Read clipboard when no image present
   - **Expected:** `read_image()` returns `Err(FormatMismatch)`

### TC-14.2.1.3 Clipboard No Frame Hitch

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |
| 2 | R-14.2.1    |

1. **#1** — Clipboard write during game loop at 60 fps
   - **Expected:** No frame exceeds 16.67 ms target budget
2. **#2** — Clipboard read during game loop at 60 fps
   - **Expected:** No frame exceeds 16.67 ms target budget

### TC-14.2.2.1 File Dialog Filter

| # | Requirement |
|---|-------------|
| 1 | R-14.2.2    |
| 2 | R-14.2.2    |

1. **#1** — Open file dialog with filter ".png"
   - **Expected:** Returned path (if any) has .png extension
2. **#2** — Open file dialog with filter ".txt,.md"
   - **Expected:** Returned path has .txt or .md extension

### TC-14.2.2.2 File Dialog Non-Blocking

| # | Requirement |
|---|-------------|
| 1 | R-14.2.2    |

1. **#1** — Open file dialog while game loop runs
   - **Expected:** Frame rate within 10% of 60 fps target

### TC-14.2.3.1 Notification Delivery

| # | Requirement |
|---|-------------|
| 1 | R-14.2.3    |

1. **#1** — Minimize window, trigger `show_notification("Title", "Body")`
   - **Expected:** OS notification API invoked with "Title" and "Body"

### TC-14.2.3.2 Tray Icon Menu Event

| # | Requirement |
|---|-------------|
| 1 | R-14.2.3    |

1. **#1** — Create tray icon with menu item id=42, simulate selection
   - **Expected:** Event received with id=42

### TC-14.2.3.3 Notification Unsupported Fallback

| # | Requirement |
|---|-------------|
| 1 | R-14.2.3    |

1. **#1** — Call `show_notification()` on platform without OS notifications
   - **Expected:** Returns `Err(OsError::Unsupported)`

### TC-14.2.4.1 Drag-Drop Accepted Extension

| # | Requirement |
|---|-------------|
| 1 | R-14.2.4    |

1. **#1** — Drag file "asset.png" onto window with filter ".png"
   - **Expected:** Drop event contains correct path

### TC-14.2.4.2 Drag-Drop Rejected Extension

| # | Requirement |
|---|-------------|
| 1 | R-14.2.4    |

1. **#1** — Drag file "script.exe" onto window with filter ".png"
   - **Expected:** Drop is rejected, no event emitted

### TC-14.2.4.3 Drag-Drop No Frame Spike

| # | Requirement |
|---|-------------|
| 1 | R-14.2.4    |

1. **#1** — Drag hover and drop during game loop
   - **Expected:** No frame time spike exceeds 2 ms above baseline

### TC-14.2.5.1 Keyboard Dead Key Compose

| # | Requirement |
|---|-------------|
| 1 | R-14.2.5    |
| 2 | R-14.2.5    |

1. **#1** — French AZERTY layout, type `^` then `e`
   - **Expected:** Engine emits composed character `ê`
2. **#2** — French AZERTY layout, type `^` then `a`
   - **Expected:** Engine emits composed character `â`

### TC-14.2.5.2 Keyboard Layout Switch

| # | Requirement |
|---|-------------|
| 1 | R-14.2.5    |

1. **#1** — Switch OS keyboard layout at runtime
   - **Expected:** `poll_layout_change()` returns new layout within one frame

### TC-14.2.6.1 IME Composition Events

| # | Requirement |
|---|-------------|
| 1 | R-14.2.6    |

1. **#1** — CJK IME active, type composition sequence for "日本"
   - **Expected:** Composition update events contain correct intermediate text

### TC-14.2.6.2 IME Commit

| # | Requirement |
|---|-------------|
| 1 | R-14.2.6    |

1. **#1** — Commit CJK IME composition for "日本"
   - **Expected:** Final committed text equals "日本"

### TC-14.2.6.3 IME Candidate Window Tracks Cursor

| # | Requirement |
|---|-------------|
| 1 | R-14.2.6    |

1. **#1** — Move and resize game window with active IME
   - **Expected:** Candidate window position updates to follow text cursor

### TC-14.4.1.1 Crash Handler Null Deref

| # | Requirement |
|---|-------------|
| 1 | R-14.4.1    |

1. **#1** — Trigger null-pointer deref in child process
   - **Expected:** Valid dump file written to crash directory

### TC-14.4.1.2 Crash Handler Stack in Dump

| # | Requirement |
|---|-------------|
| 1 | R-14.4.1    |

1. **#1** — Load dump file via debugger API after crash
   - **Expected:** Faulting thread's stack and registers present

### TC-14.4.1.3 Crash Handler Corrupt Heap

| # | Requirement |
|---|-------------|
| 1 | R-14.4.1    |

1. **#1** — Corrupt heap then crash in child process
   - **Expected:** Out-of-process capture produces valid dump

### TC-14.4.2.1 Symbol Upload and Symbolication

| # | Requirement |
|---|-------------|
| 1 | R-14.4.2    |

1. **#1** — Build release binary, upload symbols, trigger crash
   - **Expected:** Symbolicated report has correct function names and line numbers

### TC-14.4.2.2 Build ID Extraction Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.4.2    |
| 2 | R-14.4.2    |
| 3 | R-14.4.2    |

1. **#1** — Extract build ID on Windows
   - **Expected:** PE GUID+age format
2. **#2** — Extract build ID on macOS
   - **Expected:** LC_UUID format
3. **#3** — Extract build ID on Linux
   - **Expected:** GNU build-id format

### TC-14.4.3.1 Crash Aggregation Clustering

| # | Requirement |
|---|-------------|
| 1 | R-14.4.3    |

1. **#1** — Submit 50 reports with 3 distinct stack signatures
   - **Expected:** 3 clusters created

### TC-14.4.3.2 Crash Aggregation Spike Alert

| # | Requirement |
|---|-------------|
| 1 | R-14.4.3    |

1. **#1** — Inject 20 reports for one cluster in 5 minutes
   - **Expected:** Alert fires for spike detection

### TC-14.4.4.1 Logging Structured Output

| # | Requirement |
|---|-------------|
| 1 | R-14.4.4    |

1. **#1** — Emit 10,000 records across 4 channels
   - **Expected:** All appear with correct timestamp, severity, channel, fields

### TC-14.4.4.2 Logging Channel Filter

| # | Requirement |
|---|-------------|
| 1 | R-14.4.4    |

1. **#1** — Filter by channel="render", severity=Warn
   - **Expected:** Non-matching records excluded from output

### TC-14.4.5.1 Perf Counter Concurrent

| # | Requirement |
|---|-------------|
| 1 | R-14.4.5    |

1. **#1** — Increment counter from 8 threads, 1M iterations each
   - **Expected:** Merged total = 8,000,000

### TC-14.4.6.1 GPU Breadcrumb Identifies Fault

| # | Requirement |
|---|-------------|
| 1 | R-14.4.6    |

1. **#1** — Inject GPU hang after marker "pass_3"
   - **Expected:** `read_last_completed()` returns "pass_3"

### TC-14.4.6.2 GPU Breadcrumb in Crash Report

| # | Requirement |
|---|-------------|
| 1 | R-14.4.6    |

1. **#1** — GPU crash with breadcrumbs enabled
   - **Expected:** Crash report includes GPU breadcrumb data alongside CPU minidump

### TC-14.6.1.1 Async Read Write Offsets

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** — Write 4 MB in 1 MB chunks at offsets 0/1M/2M/3M from 4 tasks
   - **Expected:** Read back all 4 MB, data integrity verified

### TC-14.6.1.2 Async Read No Blocking

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** — Submit 10 MB async read
   - **Expected:** No worker thread blocks; data integrity check passes

### TC-14.6.1.3 No Stdlib File IO

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** — Static analysis of codebase
   - **Expected:** Zero `std::fs` or `std::io::Read`/`Write` calls found

### TC-14.6.2.1 Create Dir Recursive

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |

1. **#1** — `create_dir_all("a/b/c")`
   - **Expected:** All 3 directory levels exist

### TC-14.6.2.2 Delete Batch Concurrent

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |

1. **#1** — Submit 100 file deletions via `delete_batch`
   - **Expected:** All 100 complete concurrently, no thread blocking

### TC-14.6.3.1 Stat Batch Correctness

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |

1. **#1** — Create 100 files with known sizes, `stat_batch` all 100
   - **Expected:** Each result matches expected size and type

### TC-14.6.4.1 Enumerate Depth Limit

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |

1. **#1** — 5-level tree with 1000 files, enumerate with max_depth=3
   - **Expected:** Only entries within depth 3 returned

### TC-14.6.4.2 Enumerate Glob Filter

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |

1. **#1** — Enumerate with glob="*.png" in dir with mixed files
   - **Expected:** Only .png files returned

### TC-14.6.5.1 Watcher Create Modify Delete Rename

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |

1. **#1** — Watch dir; create, modify, rename, delete a file
   - **Expected:** Correctly typed event for each operation

### TC-14.6.5.2 Watcher Debounce

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |

1. **#1** — Perform 100 rapid writes to same file
   - **Expected:** Debounce coalesces into fewer events

### TC-14.6.6.1 Content Hash False Positive

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |

1. **#1** — Touch file without changing content
   - **Expected:** No change event emitted downstream

### TC-14.6.6.2 Content Hash Genuine Change

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |

1. **#1** — Modify file content
   - **Expected:** Change event fires

### TC-14.6.6.3 Content Hash Rename Into Place Same

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |

1. **#1** — Write via rename-into-place with identical content
   - **Expected:** No false positive event

### TC-14.6.6.4 Content Hash Rename Into Place Diff

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |

1. **#1** — Write via rename-into-place with different content
   - **Expected:** Genuine change detected

### TC-14.6.7.1 Canonical Path Relative

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |

1. **#1** — `CanonicalPath::resolve("./foo/../bar")`
   - **Expected:** Correct canonical absolute path to "bar"

### TC-14.6.7.2 Canonical Path Symlink

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |

1. **#1** — Resolve symlink pointing to real file
   - **Expected:** Canonical path matches target's real path

### TC-14.6.7.3 Canonical Path Alias Dedup

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |

1. **#1** — Two aliased paths as asset keys
   - **Expected:** Both map to same canonical entry

## Integration Tests

### TC-14.2.5.I1 Keyboard Layout Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.2.5    |
| 2 | R-14.2.5    |
| 3 | R-14.2.5    |

1. **#1** — French AZERTY on Windows
   - **Expected:** Dead-key compose produces correct characters
2. **#2** — French AZERTY on macOS
   - **Expected:** Dead-key compose produces correct characters
3. **#3** — French AZERTY on Linux (xkbcommon)
   - **Expected:** Dead-key compose produces correct characters

### TC-14.6.5.I1 Watcher Recursive

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |

1. **#1** — Enable recursive watch, modify file in nested subdir
   - **Expected:** Event captured from nested subdirectory

### TC-14.6.7.I1 Canonical Path Platform Specifics

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |
| 2 | R-14.6.7    |

1. **#1** — On Windows: UNC path and `\\?\` long-path prefix
   - **Expected:** Resolves correctly to canonical form
2. **#2** — On macOS: two case-variant paths to same file
   - **Expected:** Both resolve to same canonical path

## Benchmarks

### TC-14.6.1.B1 Async Read Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sequential 1 GB async read | Throughput | >= 80% raw disk bandwidth | R-14.6.1 |

### TC-14.6.1.B2 Async Write Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sequential 1 GB async write | Throughput | >= 80% raw disk bandwidth | R-14.6.1 |

### TC-14.4.4.B1 Log Emission Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Emit 100K log records | Per-record latency | < 1 us | R-14.4.4 |

### TC-14.4.5.B1 Counter Increment Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Increment lock-free counter 10M times | Per-increment latency | < 50 ns | R-14.4.5 |

### TC-14.2.1.B1 Clipboard Round-Trip Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Write + read text clipboard | Total latency | < 1 frame budget | R-14.2.1 |

### TC-14.6.5.B1 File Watcher Event Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify file, measure time to consumer callback | Latency | < 100 ms | R-14.6.5 |

### TC-14.6.6.B1 Content Hash Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BLAKE3 hash 1 GB file | Throughput | >= 2 GB/s | R-14.6.6 |
