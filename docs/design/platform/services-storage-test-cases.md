# Platform Services and Storage Test Cases

Companion test cases for [services-storage.md](services-storage.md).

## Unit Tests

### TC-14.5.1.1 Achievement Unlock Online

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlock(AchievementId(1))` while online | State = `Unlocked`, platform API called | R-14.5.1 |

### TC-14.5.1.2 Achievement Unlock Offline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlock(AchievementId(1))` while offline | Enqueued in deferred queue, pending_count = 1 | R-14.5.1 |

### TC-14.5.1.3 Achievement Deferred Retry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue unlock, call `drain_ready(now + backoff)` | Retry submitted to platform | R-14.5.1 |

### TC-14.5.1.4 Achievement Max Retries

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fail unlock max_retries+1 times | Entry dropped from deferred queue | R-14.5.1 |

### TC-14.5.1.5 Achievement Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `sync()` with platform returning 3 unlocked | Local state shows 3 unlocked achievements | R-14.5.1 |

### TC-14.5.2.1 Leaderboard Submit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `submit_score(board_id, 9500)` | Score recorded, `Ok(())` returned | R-14.5.2 |

### TC-14.5.2.2 Leaderboard Cache TTL

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query leaderboard twice within TTL | Second call returns cached result, 0 API calls | R-14.5.2 |

### TC-14.5.2.3 Leaderboard Rate Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 50 scores in 1 second | `Err(LeaderboardError::RateLimited)` after limit | R-14.5.2 |

### TC-14.5.3.1 Presence Throttle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call `update()` 10 times in 5 seconds | Only 1 platform API call made | R-14.5.3 |

### TC-14.5.3.2 Presence Latest Wins

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rapid updates: "In Menu", "In Combat", "In Town" | Platform receives "In Town" (last state) | R-14.5.3 |

### TC-14.5.3.3 Presence Clear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `clear()` | Platform clear API called | R-14.5.3 |

### TC-14.5.5.1 Cloud Upload Download

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload 1 KB data, download by same key | Downloaded data identical to uploaded | R-14.5.5 |

### TC-14.5.5.2 Cloud Conflict Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Local timestamp=100, remote timestamp=200 | Returns `ConflictResolution::Conflict` | R-14.5.5 |

### TC-14.5.5.3 Cloud No Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Local timestamp=200, remote timestamp=200 | Returns `ConflictResolution::NoConflict` | R-14.5.5 |

### TC-14.5.5.4 Cloud Quota Exceeded

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload data exceeding quota | `Err(CloudError::QuotaExceeded)` | R-14.5.5 |

### TC-14.5.6.1 Entitlement Owned

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query owned DLC entitlement | `is_owned()` returns true | R-14.5.6 |

### TC-14.5.6.2 Entitlement Subscription Expired

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query expired subscription | `is_subscription_active()` returns false | R-14.5.6 |

### TC-14.5.8.1 Prefs Get Default

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `get(PrefKey { key: "volume", default: Float(0.8) })` on empty store | Returns `Float(0.8)` | R-14.5.8 |

### TC-14.5.8.2 Prefs Set Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set("volume", Float(0.5))` then `get("volume")` | Returns `Float(0.5)` | R-14.5.8 |

### TC-14.5.8.3 Prefs Atomic Write

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `save()`, kill process mid-write | File not corrupted, previous state intact | R-14.5.8 |

### TC-14.5.8.4 Prefs Reset Defaults

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set 5 prefs, `reset_to_defaults()` | All values match defaults | R-14.5.8 |

### TC-14.5.9.1 Cache Put Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `put("key1", AssetBundle, 1KB_data)` then `get("key1")` | Retrieved data matches stored data | R-14.5.9 |

### TC-14.5.9.2 Cache LRU Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill cache to budget, put new entry | Least-recently-used entry evicted | R-14.5.9 |

### TC-14.5.9.3 Cache Clear Category

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `clear_category(StreamingData)` | Only StreamingData entries removed | R-14.5.9 |

### TC-14.5.9.4 Cache Priority Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill with GenerationOutput + AssetBundle entries, exceed budget | GenerationOutput (priority 0) evicted first | R-14.5.9 |

### TC-14.5.10.1 Dev Cache Local Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `store(hash, CompiledAsset, data)` then `lookup(hash)` | Returns `(CacheHitTier::Local, Some(data))` | R-14.5.10 |

### TC-14.5.10.2 Dev Cache Network Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Local miss, network has artifact | Returns `(CacheHitTier::SharedNetwork, Some(data))` | R-14.5.10 |

### TC-14.5.10.3 Dev Cache Miss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lookup hash not in local or network | Returns `(CacheHitTier::Miss, None)` | R-14.5.10 |

### TC-14.5.10.4 Dev Cache Hash Invalidation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store with hash_a, lookup with hash_b | Returns Miss | R-14.5.10 |

### TC-14.5.11.1 PSO Store Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `store(pso_key, blob_data)` then `get(pso_key)` | Retrieved data matches stored blob | R-14.5.11 |

### TC-14.5.11.2 PSO Driver Invalidation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change `GpuDriverKey.driver_version`, call `load_all` | Old entries invalidated, count = 0 | R-14.5.11 |

### TC-14.5.12.1 Temp RAII Cleanup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `allocate("tmp.dat")`, write data, drop handle | File deleted from filesystem | R-14.5.12 |

### TC-14.5.12.2 Temp Orphan Cleanup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create file > 24 hours old in temp dir, call `init()` | Old file cleaned up | R-14.5.12 |

### TC-14.5.12.3 Temp Budget Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill to max_bytes budget, allocate new file | Oldest temp file evicted | R-14.5.12 |

### TC-14.5.8.5 Platform Paths Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `PlatformPaths::preferences("MyGame")` on Windows | Path starts with `%LOCALAPPDATA%\Harmonius\MyGame\` | R-14.5.8 |

### TC-14.5.8.6 Platform Paths macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `PlatformPaths::preferences("MyGame")` on macOS | Path starts with `~/Library/Application Support/MyGame/` | R-14.5.8 |

### TC-14.5.8.7 Platform Paths Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `PlatformPaths::preferences("MyGame")` on Linux | Path starts with `$XDG_DATA_HOME/MyGame/` | R-14.5.8 |

## Integration Tests

### TC-14.5.1.I1 Steam Achievement Unlock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unlock via Steamworks SDK | Achievement visible in Steam client | R-14.5.1 |

### TC-14.5.2.I1 Steam Leaderboard Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit score 10000 via Steamworks, query rank | Rank reflects submitted score | R-14.5.2 |

### TC-14.5.5.I1 Steam Cloud Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload via ISteamRemoteStorage, download | Data integrity verified | R-14.5.5 |

### TC-14.5.8.I1 Prefs Cloud Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save prefs on device A, load on device B | Synced preferences match | R-14.5.8 |

### TC-14.5.11.I1 PSO Warmup Eliminates Stutter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play scene, relaunch with PSO cache | Zero shader compilation stutter on relaunch | R-14.5.11 |

### TC-14.5.9.I1 Cache 10 GB Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill 10 GB cache, download new content | Eviction completes, new content stored | R-14.5.9 |

### TC-14.5.7.I1 Console Suspend Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger console suspend event | State saved within platform deadline | R-14.5.7 |

### TC-14.5.1.I2 Deferred Queue Across Restart

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue unlock, shutdown, restart | Queue persisted and retry submitted | R-14.5.1 |

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
