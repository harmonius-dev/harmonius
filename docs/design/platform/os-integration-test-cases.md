# Platform OS Integration Test Cases

Companion test cases for [os-integration.md](os-integration.md).

## Unit Tests

### TC-14.2.1.1 Clipboard Text Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write UTF-8 string "Hello 世界 🌍" to clipboard | `read_text()` returns identical string | R-14.2.1 |
| 2 | Write empty string "" to clipboard | `read_text()` returns `Ok("")` | R-14.2.1 |
| 3 | Write 1 MB UTF-8 string to clipboard | `read_text()` returns identical string | R-14.2.1 |

### TC-14.2.1.2 Clipboard Image Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 256x256 RGBA image to clipboard | `read_image()` returns matching pixel data within tolerance | R-14.2.1 |
| 2 | Read clipboard when no image present | `read_image()` returns `Err(FormatMismatch)` | R-14.2.1 |

### TC-14.2.1.3 Clipboard No Frame Hitch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Clipboard write during game loop at 60 fps | No frame exceeds 16.67 ms target budget | R-14.2.1 |
| 2 | Clipboard read during game loop at 60 fps | No frame exceeds 16.67 ms target budget | R-14.2.1 |

### TC-14.2.2.1 File Dialog Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open file dialog with filter ".png" | Returned path (if any) has .png extension | R-14.2.2 |
| 2 | Open file dialog with filter ".txt,.md" | Returned path has .txt or .md extension | R-14.2.2 |

### TC-14.2.2.2 File Dialog Non-Blocking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open file dialog while game loop runs | Frame rate within 10% of 60 fps target | R-14.2.2 |

### TC-14.2.3.1 Notification Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Minimize window, trigger `show_notification("Title", "Body")` | OS notification API invoked with "Title" and "Body" | R-14.2.3 |

### TC-14.2.3.2 Tray Icon Menu Event

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create tray icon with menu item id=42, simulate selection | Event received with id=42 | R-14.2.3 |

### TC-14.2.3.3 Notification Unsupported Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call `show_notification()` on platform without OS notifications | Returns `Err(OsError::Unsupported)` | R-14.2.3 |

### TC-14.2.4.1 Drag-Drop Accepted Extension

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag file "asset.png" onto window with filter ".png" | Drop event contains correct path | R-14.2.4 |

### TC-14.2.4.2 Drag-Drop Rejected Extension

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag file "script.exe" onto window with filter ".png" | Drop is rejected, no event emitted | R-14.2.4 |

### TC-14.2.4.3 Drag-Drop No Frame Spike

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag hover and drop during game loop | No frame time spike exceeds 2 ms above baseline | R-14.2.4 |

### TC-14.2.5.1 Keyboard Dead Key Compose

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | French AZERTY layout, type `^` then `e` | Engine emits composed character `ê` | R-14.2.5 |
| 2 | French AZERTY layout, type `^` then `a` | Engine emits composed character `â` | R-14.2.5 |

### TC-14.2.5.2 Keyboard Layout Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch OS keyboard layout at runtime | `poll_layout_change()` returns new layout within one frame | R-14.2.5 |

### TC-14.2.6.1 IME Composition Events

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | CJK IME active, type composition sequence for "日本" | Composition update events contain correct intermediate text | R-14.2.6 |

### TC-14.2.6.2 IME Commit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Commit CJK IME composition for "日本" | Final committed text equals "日本" | R-14.2.6 |

### TC-14.2.6.3 IME Candidate Window Tracks Cursor

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move and resize game window with active IME | Candidate window position updates to follow text cursor | R-14.2.6 |

### TC-14.4.1.1 Crash Handler Null Deref

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger null-pointer deref in child process | Valid dump file written to crash directory | R-14.4.1 |

### TC-14.4.1.2 Crash Handler Stack in Dump

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load dump file via debugger API after crash | Faulting thread's stack and registers present | R-14.4.1 |

### TC-14.4.1.3 Crash Handler Corrupt Heap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Corrupt heap then crash in child process | Out-of-process capture produces valid dump | R-14.4.1 |

### TC-14.4.2.1 Symbol Upload and Symbolication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build release binary, upload symbols, trigger crash | Symbolicated report has correct function names and line numbers | R-14.4.2 |

### TC-14.4.2.2 Build ID Extraction Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Extract build ID on Windows | PE GUID+age format | R-14.4.2 |
| 2 | Extract build ID on macOS | LC_UUID format | R-14.4.2 |
| 3 | Extract build ID on Linux | GNU build-id format | R-14.4.2 |

### TC-14.4.3.1 Crash Aggregation Clustering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 50 reports with 3 distinct stack signatures | 3 clusters created | R-14.4.3 |

### TC-14.4.3.2 Crash Aggregation Spike Alert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject 20 reports for one cluster in 5 minutes | Alert fires for spike detection | R-14.4.3 |

### TC-14.4.4.1 Logging Structured Output

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emit 10,000 records across 4 channels | All appear with correct timestamp, severity, channel, fields | R-14.4.4 |

### TC-14.4.4.2 Logging Channel Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Filter by channel="render", severity=Warn | Non-matching records excluded from output | R-14.4.4 |

### TC-14.4.5.1 Perf Counter Concurrent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Increment counter from 8 threads, 1M iterations each | Merged total = 8,000,000 | R-14.4.5 |

### TC-14.4.6.1 GPU Breadcrumb Identifies Fault

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject GPU hang after marker "pass_3" | `read_last_completed()` returns "pass_3" | R-14.4.6 |

### TC-14.4.6.2 GPU Breadcrumb in Crash Report

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | GPU crash with breadcrumbs enabled | Crash report includes GPU breadcrumb data alongside CPU minidump | R-14.4.6 |

### TC-14.6.1.1 Async Read Write Offsets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 4 MB in 1 MB chunks at offsets 0/1M/2M/3M from 4 tasks | Read back all 4 MB, data integrity verified | R-14.6.1 |

### TC-14.6.1.2 Async Read No Blocking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 10 MB async read | No worker thread blocks; data integrity check passes | R-14.6.1 |

### TC-14.6.1.3 No Stdlib File IO

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Static analysis of codebase | Zero `std::fs` or `std::io::Read`/`Write` calls found | R-14.6.1 |

### TC-14.6.2.1 Create Dir Recursive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create_dir_all("a/b/c")` | All 3 directory levels exist | R-14.6.2 |

### TC-14.6.2.2 Delete Batch Concurrent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 100 file deletions via `delete_batch` | All 100 complete concurrently, no thread blocking | R-14.6.2 |

### TC-14.6.3.1 Stat Batch Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create 100 files with known sizes, `stat_batch` all 100 | Each result matches expected size and type | R-14.6.3 |

### TC-14.6.4.1 Enumerate Depth Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-level tree with 1000 files, enumerate with max_depth=3 | Only entries within depth 3 returned | R-14.6.4 |

### TC-14.6.4.2 Enumerate Glob Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enumerate with glob="*.png" in dir with mixed files | Only .png files returned | R-14.6.4 |

### TC-14.6.5.1 Watcher Create Modify Delete Rename

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Watch dir; create, modify, rename, delete a file | Correctly typed event for each operation | R-14.6.5 |

### TC-14.6.5.2 Watcher Debounce

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Perform 100 rapid writes to same file | Debounce coalesces into fewer events | R-14.6.5 |

### TC-14.6.6.1 Content Hash False Positive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Touch file without changing content | No change event emitted downstream | R-14.6.6 |

### TC-14.6.6.2 Content Hash Genuine Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify file content | Change event fires | R-14.6.6 |

### TC-14.6.6.3 Content Hash Rename Into Place Same

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write via rename-into-place with identical content | No false positive event | R-14.6.6 |

### TC-14.6.6.4 Content Hash Rename Into Place Diff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write via rename-into-place with different content | Genuine change detected | R-14.6.6 |

### TC-14.6.7.1 Canonical Path Relative

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `CanonicalPath::resolve("./foo/../bar")` | Correct canonical absolute path to "bar" | R-14.6.7 |

### TC-14.6.7.2 Canonical Path Symlink

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Resolve symlink pointing to real file | Canonical path matches target's real path | R-14.6.7 |

### TC-14.6.7.3 Canonical Path Alias Dedup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two aliased paths as asset keys | Both map to same canonical entry | R-14.6.7 |

## Integration Tests

### TC-14.2.5.I1 Keyboard Layout Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | French AZERTY on Windows | Dead-key compose produces correct characters | R-14.2.5 |
| 2 | French AZERTY on macOS | Dead-key compose produces correct characters | R-14.2.5 |
| 3 | French AZERTY on Linux (xkbcommon) | Dead-key compose produces correct characters | R-14.2.5 |

### TC-14.6.5.I1 Watcher Recursive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable recursive watch, modify file in nested subdir | Event captured from nested subdirectory | R-14.6.5 |

### TC-14.6.7.I1 Canonical Path Platform Specifics

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | On Windows: UNC path and `\\?\` long-path prefix | Resolves correctly to canonical form | R-14.6.7 |
| 2 | On macOS: two case-variant paths to same file | Both resolve to same canonical path | R-14.6.7 |

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
