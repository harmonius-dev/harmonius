# Shared Asset Cache Test Cases

Companion test cases for [shared-cache.md](shared-cache.md).

## Unit Tests

### TC-15.11.1.1 Cache Key Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — `CacheKey::compute(source, settings, "v1.0")` called twice with identical inputs
   - **Expected:** both keys byte-equal

### TC-15.11.1.2 Cache Key Changes on Source

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — `compute(source_a, settings, "v1.0")` vs `compute(source_b, settings, "v1.0")`
   - **Expected:** keys differ

### TC-15.11.5.1 Cache Key Changes on Tool Version

| # | Requirement |
|---|-------------|
| 1 | R-15.11.5   |

1. **#1** — `compute(source, settings, "v1.0")` vs `compute(source, settings, "v2.0")`
   - **Expected:** keys differ

### TC-15.11.1.3 Cache Key Changes on Settings

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — `compute(source, settings_debug, "v1.0")` vs `compute(source, settings_release, "v1.0")`
   - **Expected:** keys differ

### TC-15.11.2.1 Shader Key per Platform

| # | Requirement |
|---|-------------|
| 1 | R-15.11.2   |

1. **#1** — `from_shader(hash, Windows, flags)` vs `from_shader(hash, MacOS, flags)`
   - **Expected:** keys differ

### TC-15.11.2.2 Shader Key per Permutation

| # | Requirement |
|---|-------------|
| 1 | R-15.11.2   |

1. **#1** — `from_shader(hash, Windows, flags_a)` vs `from_shader(hash, Windows, flags_b)`
   - **Expected:** keys differ

### TC-15.11.3.1 Graph Bytecode Platform-Agnostic

| # | Requirement |
|---|-------------|
| 1 | R-15.11.3   |

1. **#1** — `from_graph(hash, None)` on Windows vs macOS
   - **Expected:** keys identical (platform-agnostic)

### TC-15.11.3.2 Graph AOT per Platform

| # | Requirement |
|---|-------------|
| 1 | R-15.11.3   |

1. **#1** — `from_graph(hash, Some(Windows))` vs `from_graph(hash, Some(MacOS))`
   - **Expected:** keys differ

### TC-15.11.1.4 Local Cache Put and Get

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |
| 2 | R-15.11.1   |
| 3 | R-15.11.1   |

1. **#1** — `put(key, entry)` then `get(key)`
   - **Expected:** returns `Some(entry)` with matching data
2. **#2** — `get(unknown_key)`
   - **Expected:** returns `None`
3. **#3** — `contains(key)` after `put`
   - **Expected:** returns `true`

### TC-15.11.5.2 Local Cache LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-15.11.5   |
| 2 | R-15.11.5   |

1. **#1** — Fill cache to `max_bytes`, add one more entry
   - **Expected:** oldest-accessed entry evicted, `size_bytes() <= max_bytes`
2. **#2** — Access entry A, then fill cache
   - **Expected:** A survives eviction (recently used), older entries evicted

### TC-15.11.6.1 Zstd Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-15.11.6   |
| 2 | R-15.11.6   |

1. **#1** — Compress 10 MB data with Zstd, decompress
   - **Expected:** decompressed data byte-identical to original
2. **#2** — Compress empty data
   - **Expected:** decompressed result is empty

### TC-15.11.1.5 Domain Tag Separation

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — `CacheKey::compute(data, settings, ver)` vs
   `CacheKey::from_shader(hash, platform, flags)` with same raw bytes
   - **Expected:** keys differ due to domain tag

### TC-15.11.8.1 Metrics Hit and Miss Count

| # | Requirement |
|---|-------------|
| 1 | R-15.11.8   |

1. **#1** — Call `record_hit()` 10 times, `record_miss()` 5 times, `snapshot()`
   - **Expected:** `hit_count == 10`, `miss_count == 5`, `hit_rate ~= 0.667`

### TC-15.11.8.2 Alert Below Threshold

| # | Requirement |
|---|-------------|
| 1 | R-15.11.8   |
| 2 | R-15.11.8   |

1. **#1** — Set threshold 0.8, record 5 hits + 5 misses (rate = 0.5)
   - **Expected:** `is_alert_active() == true`
2. **#2** — Set threshold 0.8, record 9 hits + 1 miss (rate = 0.9)
   - **Expected:** `is_alert_active() == false`

### TC-15.11.5.3 GC Removes Unreferenced

| # | Requirement |
|---|-------------|
| 1 | R-15.11.5   |

1. **#1** — Store 10 entries, mark 3 as unreferenced by any branch, run GC
   - **Expected:** `GcResult.entries_deleted == 3`

### TC-15.11.5.4 GC Preserves Referenced

| # | Requirement |
|---|-------------|
| 1 | R-15.11.5   |

1. **#1** — Store 10 entries, all referenced by active branches, run GC
   - **Expected:** `GcResult.entries_deleted == 0`

## Integration Tests

### TC-15.11.1.I1 L1 Miss L2 Hit

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — Entry exists in L2 only, `resolve(key)`
   - **Expected:** returns `Some(entry)`, entry now in L1 cache

### TC-15.11.1.I2 L1 Miss L2 Miss Build

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — Entry not in L1 or L2, `resolve(key)`
   - **Expected:** returns `None`, local build triggers, `store(key, entry)` populates both L1 and
     L2

### TC-15.11.7.1 Idempotent Upload

| # | Requirement |
|---|-------------|
| 1 | R-15.11.7   |

1. **#1** — Upload entry with key K, upload same key K again
   - **Expected:** second upload returns `Ok(())`, no overwrite

### TC-15.11.7.2 CI Population All Platforms

| # | Requirement |
|---|-------------|
| 1 | R-15.11.7   |

1. **#1** — CLI populates entries for Windows, macOS, Linux
   - **Expected:** `exists(key_win) && exists(key_mac) && exists(key_linux)` all true

### TC-15.11.4.1 Prefetch Parallel

| # | Requirement |
|---|-------------|
| 1 | R-15.11.4   |

1. **#1** — `prefetch_all(100 keys)` with concurrency=8
   - **Expected:** all 100 entries in L1, max 8 concurrent downloads observed

### TC-15.11.6.2 Bandwidth Limit Respected

| # | Requirement |
|---|-------------|
| 1 | R-15.11.6   |

1. **#1** — Set `bandwidth_limit = 1_000_000` (1 MB/s), download 10 MB
   - **Expected:** download takes >= 9 seconds

### TC-15.11.4.2 First Launch Under 10 Minutes

| # | Requirement |
|---|-------------|
| 1 | R-15.11.4   |

1. **#1** — New clone with warm L2 cache, run prefetch
   - **Expected:** all assets available within 10 minutes

### TC-15.11.1.I3 Integrity Check on Download

| # | Requirement |
|---|-------------|
| 1 | R-15.11.1   |

1. **#1** — Corrupt bytes in transit (flip bit), download entry
   - **Expected:** `Err(CacheError::IntegrityError { expected, actual })`

### TC-15.11.5.I1 GC Scheduled Run

| # | Requirement |
|---|-------------|
| 1 | R-15.11.5   |

1. **#1** — Configure GC schedule, wait for trigger, check results
   - **Expected:** `GcResult.bytes_freed > 0` for entries past retention

### TC-15.11.8.I1 Metrics OTel Export

| # | Requirement |
|---|-------------|
| 1 | R-15.11.8   |

1. **#1** — Record hits/misses, call `export_otel()`
   - **Expected:** returns valid `Vec<OtelMetric>` with correct counter values

### TC-15.11.6.I1 Platform HTTP Client

| # | Requirement |
|---|-------------|
| 1 | R-15.11.6   |
| 2 | R-15.11.6   |
| 3 | R-15.11.6   |

1. **#1** — Upload and fetch entry on macOS (NSURLSession)
   - **Expected:** round-trip succeeds, data matches
2. **#2** — Upload and fetch entry on Windows (WinHTTP)
   - **Expected:** round-trip succeeds, data matches
3. **#3** — Upload and fetch entry on Linux (libcurl)
   - **Expected:** round-trip succeeds, data matches

## Benchmarks

### TC-15.11.1.B1 BLAKE3 Hash

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hash 100 MB file with BLAKE3 | latency | < 50 ms | R-15.11.1 |

### TC-15.11.1.B2 L1 Cache Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `LocalCache::get(key)` for cached entry | latency | < 1 ms | R-15.11.1 |

### TC-15.11.1.B3 L2 Fetch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Fetch 10 MB entry from L2 over LAN | latency | < 500 ms | R-15.11.1 |

### TC-15.11.6.B1 Zstd Compress

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compress 10 MB with Zstd level 3 | latency | < 50 ms | R-15.11.6 |

### TC-15.11.6.B2 Zstd Decompress

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Decompress 10 MB Zstd blob | latency | < 20 ms | R-15.11.6 |

### TC-15.11.4.B1 Prefetch Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Prefetch 1000 entries on 1 Gbps link | latency | < 60 s | R-15.11.4 |

### TC-15.11.4.B2 First Launch Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | First launch with fully warm L2 cache | wall time | < 10 min | R-15.11.4 |

### TC-15.11.8.B1 Steady-State Hit Rate

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 developers, 1 week steady-state usage | hit rate | >= 95% | R-15.11.8 |
