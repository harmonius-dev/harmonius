# Platform Services and Storage Test Cases

Companion test cases for [services-storage.md](services-storage.md).

## Unit Tests

### TC-14.5.1.1 Achievement Unlock Online

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — `unlock(AchievementId(1))` while online
   - **Expected:** State = `Unlocked`, platform API called

### TC-14.5.1.2 Achievement Unlock Offline

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — `unlock(AchievementId(1))` while offline
   - **Expected:** Enqueued in deferred queue, pending_count = 1

### TC-14.5.1.3 Achievement Deferred Retry

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — Enqueue unlock, call `drain_ready(now + backoff)`
   - **Expected:** Retry submitted to platform

### TC-14.5.1.4 Achievement Max Retries

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — Fail unlock max_retries+1 times
   - **Expected:** Entry dropped from deferred queue

### TC-14.5.1.5 Achievement Sync

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — `sync()` with platform returning 3 unlocked
   - **Expected:** Local state shows 3 unlocked achievements

### TC-14.5.2.1 Leaderboard Submit

| # | Requirement |
|---|-------------|
| 1 | R-14.5.2    |

1. **#1** — `submit_score(board_id, 9500)`
   - **Expected:** Score recorded, `Ok(())` returned

### TC-14.5.2.2 Leaderboard Cache TTL

| # | Requirement |
|---|-------------|
| 1 | R-14.5.2    |

1. **#1** — Query leaderboard twice within TTL
   - **Expected:** Second call returns cached result, 0 API calls

### TC-14.5.2.3 Leaderboard Rate Limit

| # | Requirement |
|---|-------------|
| 1 | R-14.5.2    |

1. **#1** — Submit 50 scores in 1 second
   - **Expected:** `Err(LeaderboardError::RateLimited)` after limit

### TC-14.5.3.1 Presence Throttle

| # | Requirement |
|---|-------------|
| 1 | R-14.5.3    |

1. **#1** — Call `update()` 10 times in 5 seconds
   - **Expected:** Only 1 platform API call made

### TC-14.5.3.2 Presence Latest Wins

| # | Requirement |
|---|-------------|
| 1 | R-14.5.3    |

1. **#1** — Rapid updates: "In Menu", "In Combat", "In Town"
   - **Expected:** Platform receives "In Town" (last state)

### TC-14.5.3.3 Presence Clear

| # | Requirement |
|---|-------------|
| 1 | R-14.5.3    |

1. **#1** — `clear()`
   - **Expected:** Platform clear API called

### TC-14.5.5.1 Cloud Upload Download

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** — Upload 1 KB data, download by same key
   - **Expected:** Downloaded data identical to uploaded

### TC-14.5.5.2 Cloud Conflict Detection

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** — Local timestamp=100, remote timestamp=200
   - **Expected:** Returns `ConflictResolution::Conflict`

### TC-14.5.5.3 Cloud No Conflict

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** — Local timestamp=200, remote timestamp=200
   - **Expected:** Returns `ConflictResolution::NoConflict`

### TC-14.5.5.4 Cloud Quota Exceeded

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** — Upload data exceeding quota
   - **Expected:** `Err(CloudError::QuotaExceeded)`

### TC-14.5.6.1 Entitlement Owned

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |

1. **#1** — Query owned DLC entitlement
   - **Expected:** `is_owned()` returns true

### TC-14.5.6.2 Entitlement Subscription Expired

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |

1. **#1** — Query expired subscription
   - **Expected:** `is_subscription_active()` returns false

### TC-14.5.8.1 Prefs Get Default

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — `get(PrefKey { key: "volume", default: Float(0.8) })` on empty store
   - **Expected:** Returns `Float(0.8)`

### TC-14.5.8.2 Prefs Set Get

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — `set("volume", Float(0.5))` then `get("volume")`
   - **Expected:** Returns `Float(0.5)`

### TC-14.5.8.3 Prefs Atomic Write

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — `save()`, kill process mid-write
   - **Expected:** File not corrupted, previous state intact

### TC-14.5.8.4 Prefs Reset Defaults

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — Set 5 prefs, `reset_to_defaults()`
   - **Expected:** All values match defaults

### TC-14.5.9.1 Cache Put Get

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** — `put("key1", AssetBundle, 1KB_data)` then `get("key1")`
   - **Expected:** Retrieved data matches stored data

### TC-14.5.9.2 Cache LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** — Fill cache to budget, put new entry
   - **Expected:** Least-recently-used entry evicted

### TC-14.5.9.3 Cache Clear Category

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** — `clear_category(StreamingData)`
   - **Expected:** Only StreamingData entries removed

### TC-14.5.9.4 Cache Priority Eviction

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** — Fill with GenerationOutput + AssetBundle entries, exceed budget
   - **Expected:** GenerationOutput (priority 0) evicted first

### TC-14.5.10.1 Dev Cache Local Hit

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |

1. **#1** — `store(hash, CompiledAsset, data)` then `lookup(hash)`
   - **Expected:** Returns `(CacheHitTier::Local, Some(data))`

### TC-14.5.10.2 Dev Cache Network Hit

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |

1. **#1** — Local miss, network has artifact
   - **Expected:** Returns `(CacheHitTier::SharedNetwork, Some(data))`

### TC-14.5.10.3 Dev Cache Miss

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |

1. **#1** — Lookup hash not in local or network
   - **Expected:** Returns `(CacheHitTier::Miss, None)`

### TC-14.5.10.4 Dev Cache Hash Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-14.5.10   |

1. **#1** — Store with hash_a, lookup with hash_b
   - **Expected:** Returns Miss

### TC-14.5.11.1 PSO Store Get

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |

1. **#1** — `store(pso_key, blob_data)` then `get(pso_key)`
   - **Expected:** Retrieved data matches stored blob

### TC-14.5.11.2 PSO Driver Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |

1. **#1** — Change `GpuDriverKey.driver_version`, call `load_all`
   - **Expected:** Old entries invalidated, count = 0

### TC-14.5.12.1 Temp RAII Cleanup

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |

1. **#1** — `allocate("tmp.dat")`, write data, drop handle
   - **Expected:** File deleted from filesystem

### TC-14.5.12.2 Temp Orphan Cleanup

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |

1. **#1** — Create file > 24 hours old in temp dir, call `init()`
   - **Expected:** Old file cleaned up

### TC-14.5.12.3 Temp Budget Eviction

| # | Requirement |
|---|-------------|
| 1 | R-14.5.12   |

1. **#1** — Fill to max_bytes budget, allocate new file
   - **Expected:** Oldest temp file evicted

### TC-14.5.8.5 Platform Paths Windows

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — `PlatformPaths::preferences("MyGame")` on Windows
   - **Expected:** Path starts with `%LOCALAPPDATA%\Harmonius\MyGame\`

### TC-14.5.8.6 Platform Paths macOS

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — `PlatformPaths::preferences("MyGame")` on macOS
   - **Expected:** Path starts with `~/Library/Application Support/MyGame/`

### TC-14.5.8.7 Platform Paths Linux

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — `PlatformPaths::preferences("MyGame")` on Linux
   - **Expected:** Path starts with `$XDG_DATA_HOME/MyGame/`

## Integration Tests

### TC-14.5.1.I1 Steam Achievement Unlock

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — Unlock via Steamworks SDK
   - **Expected:** Achievement visible in Steam client

### TC-14.5.2.I1 Steam Leaderboard Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-14.5.2    |

1. **#1** — Submit score 10000 via Steamworks, query rank
   - **Expected:** Rank reflects submitted score

### TC-14.5.5.I1 Steam Cloud Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** — Upload via ISteamRemoteStorage, download
   - **Expected:** Data integrity verified

### TC-14.5.8.I1 Prefs Cloud Sync

| # | Requirement |
|---|-------------|
| 1 | R-14.5.8    |

1. **#1** — Save prefs on device A, load on device B
   - **Expected:** Synced preferences match

### TC-14.5.11.I1 PSO Warmup Eliminates Stutter

| # | Requirement |
|---|-------------|
| 1 | R-14.5.11   |

1. **#1** — Play scene, relaunch with PSO cache
   - **Expected:** Zero shader compilation stutter on relaunch

### TC-14.5.9.I1 Cache 10 GB Eviction

| # | Requirement |
|---|-------------|
| 1 | R-14.5.9    |

1. **#1** — Fill 10 GB cache, download new content
   - **Expected:** Eviction completes, new content stored

### TC-14.5.7.I1 Console Suspend Resume

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |

1. **#1** — Trigger console suspend event
   - **Expected:** State saved within platform deadline

### TC-14.5.1.I2 Deferred Queue Across Restart

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |

1. **#1** — Enqueue unlock, shutdown, restart
   - **Expected:** Queue persisted and retry submitted

## Benchmarks

### TC-14.5.1.B1 Achievement Unlock Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Unlock achievement (online) | End-to-end latency | < 50 ms | R-14.5.1 |

### TC-14.5.2.B1 Leaderboard Query Cached

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Query cached leaderboard | Response time | < 1 ms | R-14.5.2 |

### TC-14.5.3.B1 Rich Presence Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update rich presence string | API call latency | < 5 ms | R-14.5.3 |

### TC-14.5.5.B1 Cloud Upload 1 KB

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Upload 1 KB to cloud storage | Round-trip latency | < 200 ms | R-14.5.5 |

### TC-14.5.8.B1 Preferences Save

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Atomic save of preferences TOML | Write + rename latency | < 50 ms | R-14.5.8 |

### TC-14.5.11.B1 PSO Cache Get

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Retrieve cached PSO blob | Lookup latency | < 1 ms | R-14.5.11 |

### TC-14.5.10.B1 BLAKE3 Hash 1 MB

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hash 1 MB data with BLAKE3 | Time | < 500 us | R-14.5.10 |

### TC-14.5.9.B1 Player Cache Eviction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evict 100 entries from player cache | Total eviction time | < 100 ms | R-14.5.9 |

### TC-14.5.12.B1 Temp File Allocate Drop

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Allocate temp file handle, drop it | Allocate + cleanup time | < 10 ms | R-14.5.12 |

### TC-14.5.10.B2 Dev Cache 3-Tier Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full 3-tier lookup (local miss, network hit) | Total lookup time | < 50 ms | R-14.5.10 |
