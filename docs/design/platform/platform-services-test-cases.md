# Platform Services Test Cases

Companion test cases for [platform-services.md](platform-services.md).

## Unit Tests

### TC-14.6.1.1 Async File Read Write Round Trip

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |
| 3 | R-14.6.1    |

1. **#1** -- Open file with `OpenFlags::create_new()`
   - **Expected:** `AsyncFile` handle returned, no error
2. **#2** -- Write 4 KB payload at offset 0, flush, close
   - **Expected:** `write()` returns `Ok(4096)`
3. **#3** -- Reopen with `OpenFlags::read_only()`, `read_to_end()`
   - **Expected:** Returned bytes match original payload

### TC-14.6.1.2 Async File Read At Offset

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |

1. **#1** -- Write 8 KB file, close, reopen read-only
   - **Expected:** File created successfully
2. **#2** -- `read(buf, offset=4096)` into 4 KB buffer
   - **Expected:** Returns `Ok(4096)`, buf matches
   second half of original data

### TC-14.6.1.3 Async File Open Nonexistent

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** -- `AsyncFile::open("missing.dat", OpenFlags::read_only())`
   - **Expected:** Returns
   `Err(FsError::NotFound { path: "missing.dat" })`

### TC-14.6.2.1 Create Dir All Nested

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |

1. **#1** -- `create_dir_all("a/b/c/d")` on empty root
   - **Expected:** All four directories exist, no error

### TC-14.6.2.2 Delete Batch

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |
| 2 | R-14.6.2    |

1. **#1** -- Create 10 temp files, call `delete_batch()` with all 10 paths
   - **Expected:** All 10 return `Ok(())`
2. **#2** -- Include 1 nonexistent path in batch of 5
   - **Expected:** 4 return `Ok(())`, 1 returns
   `Err(FsError::NotFound)`

### TC-14.6.3.1 Stat Returns Correct Metadata

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |

1. **#1** -- Create 1 KB file, call `stat()`
   - **Expected:** `file_type == File`, `size == 1024`,
   `read_only == false`
2. **#2** -- Create directory, call `stat()`
   - **Expected:** `file_type == Directory`

### TC-14.6.3.2 Stat Batch Multiple Files

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |

1. **#1** -- Create 5 files of varying sizes, `stat_batch()` all 5
   - **Expected:** 5 results, each size matches

### TC-14.6.4.1 Directory Enumerate With Glob

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |
| 2 | R-14.6.4    |

1. **#1** -- Create dir with 3 `.txt` and 2 `.png` files, enumerate with glob `"*.txt"`
   - **Expected:** Stream yields exactly 3 entries
2. **#2** -- Enumerate with `max_depth: 0` on nested structure
   - **Expected:** Only immediate children returned

### TC-14.6.5.1 File Watcher Debounce

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |

1. **#1** -- Watch dir with `debounce_ms: 100`, write file 5 times in 50 ms
   - **Expected:** Single `Modified` event received
2. **#2** -- Write file, wait 200 ms, write again
   - **Expected:** Two separate `Modified` events

### TC-14.6.5.2 File Watcher Create Delete Events

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |

1. **#1** -- Watch dir, create new file
   - **Expected:** `FileEventKind::Created` event with
   correct path
2. **#2** -- Delete the file
   - **Expected:** `FileEventKind::Deleted` event with
   correct path

### TC-14.6.5.3 File Watcher Unwatch Stops Events

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |

1. **#1** -- Watch dir, `unwatch(id)`, modify file
   - **Expected:** No events received after unwatch

### TC-14.6.6.1 Content Hash BLAKE3 Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |

1. **#1** -- Hash same 1 MB file twice via `hash_file()`
   - **Expected:** Both `Blake3Hash` values identical
2. **#2** -- Modify 1 byte, hash again
   - **Expected:** Hash differs from original

### TC-14.6.6.2 Content Hash Change Detection

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |

1. **#1** -- Hash file, call `has_content_changed(path, hash)`
   - **Expected:** Returns `Ok(false)`
2. **#2** -- Modify file, call `has_content_changed(path, old_hash)`
   - **Expected:** Returns `Ok(true)`

### TC-14.6.7.1 Canonical Path Resolve

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |
| 2 | R-14.6.7    |

1. **#1** -- `CanonicalPath::resolve("a/../b/./c")`
   - **Expected:** Returns normalized path `"b/c"`
2. **#2** -- `CanonicalPath::resolve("")`
   - **Expected:** Returns `Err(FsError)`

### TC-14.6.7.2 Canonical Path Join And Components

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |
| 2 | R-14.6.7    |
| 3 | R-14.6.7    |

1. **#1** -- `path.join("subdir")`
   - **Expected:** Returns valid `CanonicalPath` with
   appended component
2. **#2** -- `path.file_name()` on `"/a/b/file.txt"`
   - **Expected:** Returns `Some("file.txt")`
3. **#3** -- `path.extension()` on `"/a/b/file.txt"`
   - **Expected:** Returns `Some("txt")`

### TC-14.2.1.1 Clipboard Write Text

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |

1. **#1** -- `clipboard.set_text("hello")`
   - **Expected:** Returns `Ok(())`, next frame
   `ClipboardState.text == Some("hello")`

### TC-14.2.1.2 Clipboard Write Image

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |

1. **#1** -- `clipboard.set_image(&ImageData { width: 2, height: 2, pixels: [0u8; 16] })`
   - **Expected:** Returns `Ok(())`, next frame
   `ClipboardState.image` contains matching data

### TC-14.2.1.3 Clipboard State Cached As ECS Resource

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |
| 2 | R-14.2.1    |

1. **#1** -- Set clipboard text, advance one frame
   - **Expected:** `Res<ClipboardState>.text ==
   Some("hello")`
2. **#2** -- Read `Res<ClipboardState>` from non-main thread system
   - **Expected:** Read succeeds, returns cached value

### TC-14.4.4.1 Logger Emit To Ring Buffer

| # | Requirement |
|---|-------------|
| 1 | R-14.4.4    |
| 2 | R-14.4.4    |

1. **#1** -- Create `Logger` with `LogSink::RingBuffer { capacity: 1024 }`, emit 10 records
   - **Expected:** All 10 records stored in ring buffer
2. **#2** -- Emit 1025 records into capacity-1024 buffer
   - **Expected:** Oldest record evicted, newest 1024
   retained

### TC-14.4.4.2 Logger Filter By Channel And Severity

| # | Requirement |
|---|-------------|
| 1 | R-14.4.4    |
| 2 | R-14.4.4    |

1. **#1** -- Set filter `("render", Severity::Warn)`, emit `Severity::Info` on channel `"render"`
   - **Expected:** Record not stored
2. **#2** -- Emit `Severity::Error` on channel `"render"`
   - **Expected:** Record stored

### TC-14.4.4.3 Logger Enum Dispatch Sinks

| # | Requirement |
|---|-------------|
| 1 | R-14.4.4    |
| 2 | R-14.4.4    |

1. **#1** -- Create logger with `LogSink::File` and `LogSink::RingBuffer`
   - **Expected:** Both sinks receive each record
2. **#2** -- Create logger with `LogSink::Platform`
   - **Expected:** Record dispatched to OS log API

### TC-14.4.5.1 Perf Counter Increment

| # | Requirement |
|---|-------------|
| 1 | R-14.4.5    |
| 2 | R-14.4.5    |

1. **#1** -- `counters.increment(&name)` 100 times, `flush()`
   - **Expected:** Snapshot contains `(name, 100.0)`
2. **#2** -- `counters.increment_by(&name, 5.0)` twice, `flush()`
   - **Expected:** Snapshot contains `(name, 10.0)`

### TC-14.4.5.2 Perf Counter Gauge

| # | Requirement |
|---|-------------|
| 1 | R-14.4.5    |

1. **#1** -- `counters.gauge(&name, 42.0)`, `counters.gauge(&name, 99.0)`, `flush()`
   - **Expected:** Snapshot contains `(name, 99.0)`

### TC-14.4.6.1 GPU Breadcrumbs Begin End Pass

| # | Requirement |
|---|-------------|
| 1 | R-14.4.6    |
| 2 | R-14.4.6    |

1. **#1** -- `begin_pass(PassId(1))`, `end_pass(PassId(1))`
   - **Expected:** `read_last_completed() ==
   Some(PassId(1))`
2. **#2** -- `begin_pass(PassId(2))` without `end_pass(PassId(2))`
   - **Expected:** `read_last_completed() ==
   Some(PassId(1))` (last completed, not current)

### TC-14.4.6.2 GPU Breadcrumbs Serialize For Crash

| # | Requirement |
|---|-------------|
| 1 | R-14.4.6    |

1. **#1** -- Complete 3 passes, call `serialize_for_crash_report()`
   - **Expected:** Non-empty `Vec<u8>` containing all
   3 pass markers

### TC-14.5.11.1 PSO Cache Synchronous Get

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |
| 2 | R-14.5.11   |

1. **#1** -- `store(key, blob)`, then `get(&key)`
   - **Expected:** Returns `Some(blob)` matching
   original data
2. **#2** -- `get(&unknown_key)`
   - **Expected:** Returns `None`

### TC-14.5.11.2 PSO Cache Invalidate All

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |

1. **#1** -- Store 5 PSO entries, `invalidate_all()`
   - **Expected:** Returns `Ok(5)`, subsequent `get()`
   for each key returns `None`

### TC-14.5.11.3 PSO Cache GPU Driver Key Isolation

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |

1. **#1** -- Create store with `GpuDriverKey { vendor: 0x10DE, device: 0x2684, driver: "535.183" }`,
   store PSO, create second store with different driver version, `load_all()`
   - **Expected:** Second store does not load first
   store's entries

### TC-14.5.12.1 Temp File Allocate And Drop

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |
| 2 | R-14.5.12   |

1. **#1** -- `manager.allocate("scratch.tmp")`
   - **Expected:** Returns `TempFileHandle` with valid
   path under manager root
2. **#2** -- Drop the handle, call `cleanup_orphans()`
   - **Expected:** File deleted, returns `Ok(1)`

### TC-14.5.12.2 Temp File Orphan Cleanup On Init

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |

1. **#1** -- Create temp files, simulate crash (skip cleanup), reinitialize
   `TempFileManager::init()`, call `cleanup_orphans()`
   - **Expected:** All orphaned files deleted

### TC-14.5.12.3 Temp File Budget Enforcement

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |

1. **#1** -- Init with `max_bytes: 1024`, allocate files totaling 2048 bytes
   - **Expected:** Returns
   `Err(TempError::BudgetExceeded)`

### TC-14.5.8.1 Preferences Get Set Round Trip

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |
| 2 | R-14.5.8    |

1. **#1** -- `prefs.set("volume", PrefValue::Float(0.8))`, then
   `prefs.get(&PrefKey { key: "volume", default: PrefValue::Float(1.0) })`
   - **Expected:** Returns `PrefValue::Float(0.8)`
2. **#2** -- `prefs.get()` for unset key with default `PrefValue::Bool(true)`
   - **Expected:** Returns `PrefValue::Bool(true)`

### TC-14.5.8.2 Preferences Dirty Flag

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |
| 2 | R-14.5.8    |

1. **#1** -- Load prefs, check `is_dirty()`
   - **Expected:** Returns `false`
2. **#2** -- `prefs.set("key", value)`, check `is_dirty()`
   - **Expected:** Returns `true`

### TC-14.5.8.3 Preferences Reset To Defaults

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** -- Set 3 keys, call `reset_to_defaults()` with those keys
   - **Expected:** All 3 return their default values

### TC-14.5.9.1 Player Cache Put Get

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** -- `cache.put("asset-1", CacheCategory::AssetBundle, &data)`, then `cache.get("asset-1")`
   - **Expected:** Returns `Ok(Some(data))` matching
   original

### TC-14.5.9.2 Player Cache LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |
| 2 | R-14.5.9    |

1. **#1** -- Set `max_bytes: 1000`, put entries totaling 1200 bytes
   - **Expected:** `evict_to_budget()` evicts oldest
   entries to fit within 1000
2. **#2** -- Check `stats().total_bytes`
   - **Expected:** `<= 1000`

### TC-14.5.9.3 Player Cache Stats Per Category

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** -- Put 3 entries across 2 categories, call `stats()`
   - **Expected:** `per_category` map has correct byte
   counts for each category

### TC-14.5.10.1 Developer Cache Store And Lookup

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |
| 2 | R-14.5.10   |

1. **#1** -- `cache.store(&hash, DevCacheCategory::CompiledAsset, &data)`, then
   `cache.lookup(&hash, DevCacheCategory::CompiledAsset)`
   - **Expected:** Returns
   `Ok((CacheHitTier::Local, Some(data)))`
2. **#2** -- Lookup with unknown hash
   - **Expected:** Returns
   `Ok((CacheHitTier::Miss, None))`

### TC-14.5.10.2 Developer Cache BLAKE3 Hash

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |

1. **#1** -- `DeveloperCache::hash(&data)` twice with same data
   - **Expected:** Both `ContentHash` values identical

### TC-14.5.1.1 Achievement Unlock Queues Request

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |
| 2 | R-14.5.1    |

1. **#1** -- `achievements.unlock(&id)` for a locked achievement
   - **Expected:** Returns `Ok(())`, state becomes
   `UnlockState::Pending`
2. **#2** -- `achievements.unlock(&id)` for already unlocked achievement
   - **Expected:** Returns
   `Err(AchievementError::AlreadyUnlocked)`

### TC-14.5.1.2 Achievement Increment Progress

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |
| 2 | R-14.5.1    |

1. **#1** -- Achievement with target=10, `increment(&id, 3)` then `state(&id)`
   - **Expected:** `current == 3`, `state == Locked`
2. **#2** -- `increment(&id, 7)` then `state(&id)`
   - **Expected:** `current == 10`,
   `state == Pending` (auto-unlock at target)

### TC-14.5.1.3 Achievement Deferred Queue Retry

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** -- Unlock while offline (backend returns error), call `flush_deferred()` after reconnect
   - **Expected:** Returns `Ok(n)` where n >= 1,
   achievement transitions to `Unlocked`

### TC-14.5.2.1 Leaderboard Submit And Query Cached

| # | Requirement |
|---|-------------|
| 1 | R-14.5.2    |
| 2 | R-14.5.2    |

1. **#1** -- `leaderboards.submit(&id, 9999)`
   - **Expected:** Returns `Ok(())`, pending count
   increments
2. **#2** -- `leaderboards.query(&id, LeaderboardScope::Global, 0, 10)` with cached data
   - **Expected:** Returns `Some(&LeaderboardResult)`
   with rows

### TC-14.5.2.2 Leaderboard Flush Pending

| # | Requirement |
|---|-------------|
| 1 | R-14.5.2    |

1. **#1** -- Submit 5 scores, `flush_pending()`
   - **Expected:** Returns `Ok(5)`, pending list empty

### TC-14.5.3.1 Rich Presence Update Throttle

| # | Requirement |
|---|-------------|
| 1 | R-14.5.3    |
| 2 | R-14.5.3    |

1. **#1** -- `presence.update(state1)`, immediately `presence.update(state2)`
   - **Expected:** First returns `Ok(())`, second is
   throttled (only latest state queued)
2. **#2** -- `presence.current()`
   - **Expected:** Returns state2 (latest)

### TC-14.5.3.2 Rich Presence Clear

| # | Requirement |
|---|-------------|
| 1 | R-14.5.3    |

1. **#1** -- Set presence, then `presence.clear()`
   - **Expected:** `presence.current()` returns `None`

### TC-14.5.5.1 Cloud Storage Upload Download

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** -- `cloud.upload(&key, &data)`, then `cloud.download(&key)`
   - **Expected:** Both return `Ok(())`

### TC-14.5.5.2 Cloud Storage Conflict Detection

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** -- `cloud.check_conflict(&key, &local_data, old_timestamp)` when cloud has newer data
   - **Expected:** Returns `Ok(())`, conflict result
   contains both local and cloud versions

### TC-14.5.5.3 Cloud Storage Quota Check

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** -- Upload data exceeding quota
   - **Expected:** Returns
   `Err(CloudError::QuotaExceeded)`

### TC-14.5.6.1 Entitlement Ownership Check

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |

1. **#1** -- After refresh, `is_owned("base-game")` for owned entitlement
   - **Expected:** Returns `true`
2. **#2** -- `is_owned("unowned-dlc")` for unowned entitlement
   - **Expected:** Returns `false`

### TC-14.5.6.2 Entitlement Subscription Active

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |

1. **#1** -- `is_subscription_active("premium")` for active subscription
   - **Expected:** Returns `true`

### TC-14.2.3.1 Notification Show

| # | Requirement |
|---|-------------|
| 1 | R-14.2.3    |

1. **#1** -- Call `show_notification()` with title="Build Done", body="Success", urgency=Normal,
   icon=None
   - **Expected:** Returns `Ok(())`

### TC-14.2.3.2 Tray Icon Create Remove

| # | Requirement |
|---|-------------|
| 1 | R-14.2.3    |
| 2 | R-14.2.3    |

1. **#1** -- `notifications.create_tray_icon( "Harmonius", &menu_items)`
   - **Expected:** Returns `Ok(TrayIconHandle)`
2. **#2** -- `notifications.remove_tray_icon(handle)`
   - **Expected:** Returns `Ok(())`

### TC-14.2.4.1 Drag Drop Filter Accepts Valid MIME

| # | Requirement |
|---|-------------|
| 1 | R-14.2.4    |

1. **#1** -- Register with `MimeFilter { mime_types: ["image/png"], extensions: ["png"] }`, simulate
   drag enter with `mime_types: ["image/png"]`
   - **Expected:** `poll_event()` returns
   `DragEvent::Enter` with matching types

### TC-14.2.4.2 Drag Drop Filter Rejects Invalid MIME

| # | Requirement |
|---|-------------|
| 1 | R-14.2.4    |

1. **#1** -- Register with filter for `"image/png"`, simulate drag with `"application/pdf"`
   - **Expected:** Event filtered out or
   `respond(DragResponse::Reject)` is appropriate

### TC-14.2.5.1 Keyboard Layout Detection

| # | Requirement |
|---|-------------|
| 1 | R-14.2.5    |

1. **#1** -- `keyboard.active_layout()`
   - **Expected:** Returns `KeyboardLayout` with
   non-empty `name` and valid `id`

### TC-14.2.5.2 Keyboard Dead Key Sequence

| # | Requirement |
|---|-------------|
| 1 | R-14.2.5    |
| 2 | R-14.2.5    |

1. **#1** -- `translate_key(dead_acute_scancode)`
   - **Expected:** Returns `DeadKeyResult::Pending`
2. **#2** -- `translate_key(e_scancode)`
   - **Expected:** Returns
   `DeadKeyResult::Composed('e' with acute)`

### TC-14.2.6.1 IME Composition Events

| # | Requirement |
|---|-------------|
| 1 | R-14.2.6    |
| 2 | R-14.2.6    |

1. **#1** -- Attach IME, simulate composition input
   - **Expected:** `poll_event()` returns
   `ImeEvent::Composition` with text and cursor
2. **#2** -- Confirm composition
   - **Expected:** `poll_event()` returns
   `ImeEvent::Commit` with final text

### TC-14.2.6.2 IME Position Update

| # | Requirement |
|---|-------------|
| 1 | R-14.2.6    |

1. **#1** -- `ime.set_position(&ImePosition { x: 100.0, y: 200.0, line_height: 16.0 })`
   - **Expected:** No error, candidate window
   repositioned

### TC-14.4.1.1 Crash Handler Install

| # | Requirement |
|---|-------------|
| 1 | R-14.4.1    |

1. **#1** -- `CrashHandler::install(config)` with valid crash dir and OOP handler path
   - **Expected:** Returns `Ok(CrashHandler)`

### TC-14.4.1.2 Crash Handler Metadata

| # | Requirement |
|---|-------------|
| 1 | R-14.4.1    |

1. **#1** -- `handler.set_metadata("version", "1.0")` then `handler.set_metadata("build", "abc123")`
   - **Expected:** Both key-value pairs stored for
   inclusion in crash dumps

### TC-14.4.2.1 Symbol Format Extraction

| # | Requirement |
|---|-------------|
| 1 | R-14.4.2    |

1. **#1** -- `SymbolUploader::extract_build_id( &binary_path)` on a valid binary
   - **Expected:** Returns platform-appropriate
   `SymbolFormat` variant (Pdb, Dsym, or Dwarf)

### TC-14.5.8.4 Platform Paths Per OS

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |
| 2 | R-14.5.8    |
| 3 | R-14.5.8    |

1. **#1** -- `PlatformPaths::preferences("mygame")`
   - **Expected:** Returns OS-appropriate path
   (e.g., `~/Library/Application Support/mygame/`    on macOS)
2. **#2** -- `PlatformPaths::player_cache("mygame")`
   - **Expected:** Returns OS-appropriate cache path
3. **#3** -- `PlatformPaths::temp("mygame")`
   - **Expected:** Returns OS-appropriate temp path

### TC-14.5.8.5 Preferences Atomic Save

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** -- Set values, `prefs.save(&cloud)`, read file back from disk
   - **Expected:** File contains valid TOML with all
   set values, no partial writes

## Integration Tests

### TC-14.6.1.I1 Async File 10 MB Read

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** -- Write 10 MB file, read via `AsyncFile::read_to_end()`
   - **Expected:** Data integrity verified, game loop
   not blocked during I/O

### TC-14.6.5.I1 File Watcher With Content Hash

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.6    |

1. **#1** -- Watch file, modify it, receive event, hash via `ContentHasher`
   - **Expected:** `has_content_changed()` returns
   `true`, confirming real content change

### TC-14.6.5.I2 File Watcher Recursive Nested

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |

1. **#1** -- Watch root with `recursive: true`, create file 3 levels deep
   - **Expected:** `FileEventKind::Created` event
   received for the deeply nested file

### TC-14.2.1.I1 Clipboard ECS Resource Sync

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |

1. **#1** -- Write text via `Clipboard::set_text()`, advance 2 frames, read `Res<ClipboardState>`
   from ECS system
   - **Expected:** `ClipboardState.text` matches
   written text

### TC-14.4.4.I1 Logger Multi Sink File And Ring

| # | Requirement |
|---|-------------|
| 1 | R-14.4.4    |

1. **#1** -- Create logger with `LogSink::File` and `LogSink::RingBuffer`, emit 100 records, flush
   - **Expected:** Log file on disk contains 100
   entries and ring buffer contains 100 entries

### TC-14.4.1.I1 Crash Handler With GPU Breadcrumbs

| # | Requirement |
|---|-------------|
| 1 | R-14.4.1    |
| 2 | R-14.4.6    |

1. **#1** -- Install crash handler, attach GPU breadcrumbs, complete 5 render passes
   - **Expected:** `serialize_for_crash_report()`
   contains markers for all 5 passes
2. **#2** -- Simulate crash, check minidump
   - **Expected:** Dump includes GPU breadcrumb data

### TC-14.5.1.I1 Achievement Offline Deferred Flow

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** -- Unlock 3 achievements while offline, then reconnect and call `flush_deferred()`
   - **Expected:** All 3 transition from `Pending` to
   `Unlocked`, deferred queue empty

### TC-14.5.5.I1 Cloud Storage Conflict Resolution

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** -- Upload data, modify cloud copy externally, call `check_conflict()` with stale timestamp
   - **Expected:** Conflict result contains both local
   and cloud data with timestamps for UI dialog

### TC-14.5.8.I1 Preferences Load Save Cloud Sync

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** -- `PreferencesStore::load()` with cloud service, set values, `save()` with cloud sync
   - **Expected:** Local TOML file and cloud storage
   both contain updated values

### TC-14.5.11.I1 PSO Cache Load All Preload

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |

1. **#1** -- Store 50 PSO blobs to disk, create new `PsoCacheStore`, `load_all()`
   - **Expected:** Returns `Ok(50)`, all 50 entries
   retrievable via synchronous `get()`

### TC-14.5.12.I1 Temp File Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |

1. **#1** -- Allocate 5 temp files, drop 3 handles, `cleanup_orphans()`, verify 2 remain
   - **Expected:** `cleanup_orphans()` returns
   `Ok(3)`, 2 active files still exist on disk

### TC-14.2.1.I2 Clipboard Thread Affinity

| # | Requirement |
|---|-------------|
| 1 | R-14.2.1    |

1. **#1** -- Call `Clipboard::set_text()` from main thread, verify OS clipboard API was invoked on
   main thread
   - **Expected:** Thread ID matches main thread

### TC-14.5.9.I1 Player Cache Eviction Under Load

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** -- Fill cache to capacity across 3 categories, put new entry, `evict_to_budget()`
   - **Expected:** LRU entries evicted first,
   `stats().total_bytes <= max_bytes`

### TC-14.5.10.I1 Developer Cache 3 Tier Lookup

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |

1. **#1** -- Store entry only in shared network cache, `lookup()` from fresh local cache
   - **Expected:** Returns
   `(CacheHitTier::SharedNetwork, Some(data))`

### TC-14.6.7.I1 Canonical Path Cross Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.6.7    |

1. **#1** -- Resolve paths with platform-specific separators (`\` on Windows, `/` on Unix)
   - **Expected:** Both produce equivalent
   `CanonicalPath` values

## Benchmarks

### TC-14.6.1.B1 Async Read Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Sequential read 1 GB via AsyncFile | Throughput | >= 80% raw disk | R-14.6.1 |

### TC-14.6.1.B2 Async Write Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Sequential write 1 GB via AsyncFile | Throughput | >= 80% raw disk | R-14.6.1 |

### TC-14.4.4.B1 Log Emission Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Emit 100K log records to ring buffer | Per-emit | < 1 us | R-14.4.4 |

### TC-14.4.5.B1 Counter Increment Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Increment lock-free counter 1M times | Per-op | < 50 ns | R-14.4.5 |

### TC-14.2.1.B1 Clipboard Round Trip Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Write text, read back from ECS cache | Latency | < 1 frame | R-14.2.1 |

### TC-14.6.5.B1 File Watcher Event Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Modify watched file, measure event delay | Latency | < 100 ms | R-14.6.5 |

### TC-14.6.6.B1 Content Hash Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | BLAKE3 hash 1 GB file | Throughput | >= 2 GB/s | R-14.6.6 |

### TC-14.6.6.B2 BLAKE3 Hash 1 MB

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | BLAKE3 hash 1 MB buffer | Latency | < 500 us | R-14.6.6 |

### TC-14.5.11.B1 PSO Cache Get Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | HashMap lookup after load_all (1000 entries) | Per-get | < 1 ms | R-14.5.11 |

### TC-14.5.8.B1 Preferences Save Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Atomic save 50 key-value pairs to TOML | Latency | < 50 ms | R-14.5.8 |

### TC-14.5.9.B1 Player Cache Eviction

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Evict 100 LRU entries from full cache | Latency | < 100 ms | R-14.5.9 |

### TC-14.5.12.B1 Temp File Allocate Drop

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Allocate + drop + cleanup_orphans 1 file | Latency | < 10 ms | R-14.5.12 |

### TC-14.5.10.B1 Developer Cache 3 Tier Lookup

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Local hit + shared miss + remote miss | Latency | < 50 ms | R-14.5.10 |

### TC-14.5.1.B1 Achievement Unlock Online

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Unlock single achievement (online) | Latency | < 50 ms | R-14.5.1 |

### TC-14.5.2.B1 Leaderboard Query Cached

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Query cached leaderboard (100 rows) | Latency | < 1 ms | R-14.5.2 |
