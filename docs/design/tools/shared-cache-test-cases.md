# Shared Asset Cache Test Cases

Companion test cases for [shared-cache.md](shared-cache.md).

## Unit Tests

### TC-15.11.1.1 Cache Key Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `CacheKey::compute(source, settings, "v1.0")` called twice with identical inputs | both keys byte-equal | R-15.11.1 |

### TC-15.11.1.2 Cache Key Changes on Source

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `compute(source_a, settings, "v1.0")` vs `compute(source_b, settings, "v1.0")` | keys differ | R-15.11.1 |

### TC-15.11.5.1 Cache Key Changes on Tool Version

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `compute(source, settings, "v1.0")` vs `compute(source, settings, "v2.0")` | keys differ | R-15.11.5 |

### TC-15.11.1.3 Cache Key Changes on Settings

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `compute(source, settings_debug, "v1.0")` vs `compute(source, settings_release, "v1.0")` | keys differ | R-15.11.1 |

### TC-15.11.2.1 Shader Key per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `from_shader(hash, Windows, flags)` vs `from_shader(hash, MacOS, flags)` | keys differ | R-15.11.2 |

### TC-15.11.2.2 Shader Key per Permutation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `from_shader(hash, Windows, flags_a)` vs `from_shader(hash, Windows, flags_b)` | keys differ | R-15.11.2 |

### TC-15.11.3.1 Graph Bytecode Platform-Agnostic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `from_graph(hash, None)` on Windows vs macOS | keys identical (platform-agnostic) | R-15.11.3 |

### TC-15.11.3.2 Graph AOT per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `from_graph(hash, Some(Windows))` vs `from_graph(hash, Some(MacOS))` | keys differ | R-15.11.3 |

### TC-15.11.1.4 Local Cache Put and Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `put(key, entry)` then `get(key)` | returns `Some(entry)` with matching data | R-15.11.1 |
| 2 | `get(unknown_key)` | returns `None` | R-15.11.1 |
| 3 | `contains(key)` after `put` | returns `true` | R-15.11.1 |

### TC-15.11.5.2 Local Cache LRU Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill cache to `max_bytes`, add one more entry | oldest-accessed entry evicted, `size_bytes() <= max_bytes` | R-15.11.5 |
| 2 | Access entry A, then fill cache | A survives eviction (recently used), older entries evicted | R-15.11.5 |

### TC-15.11.6.1 Zstd Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 10 MB data with Zstd, decompress | decompressed data byte-identical to original | R-15.11.6 |
| 2 | Compress empty data | decompressed result is empty | R-15.11.6 |

### TC-15.11.1.5 Domain Tag Separation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `CacheKey::compute(data, settings, ver)` vs `CacheKey::from_shader(hash, platform, flags)` with same raw bytes | keys differ due to domain tag | R-15.11.1 |

### TC-15.11.8.1 Metrics Hit and Miss Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call `record_hit()` 10 times, `record_miss()` 5 times, `snapshot()` | `hit_count == 10`, `miss_count == 5`, `hit_rate ~= 0.667` | R-15.11.8 |

### TC-15.11.8.2 Alert Below Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set threshold 0.8, record 5 hits + 5 misses (rate = 0.5) | `is_alert_active() == true` | R-15.11.8 |
| 2 | Set threshold 0.8, record 9 hits + 1 miss (rate = 0.9) | `is_alert_active() == false` | R-15.11.8 |

### TC-15.11.5.3 GC Removes Unreferenced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store 10 entries, mark 3 as unreferenced by any branch, run GC | `GcResult.entries_deleted == 3` | R-15.11.5 |

### TC-15.11.5.4 GC Preserves Referenced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store 10 entries, all referenced by active branches, run GC | `GcResult.entries_deleted == 0` | R-15.11.5 |

## Integration Tests

### TC-15.11.1.I1 L1 Miss L2 Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entry exists in L2 only, `resolve(key)` | returns `Some(entry)`, entry now in L1 cache | R-15.11.1 |

### TC-15.11.1.I2 L1 Miss L2 Miss Build

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entry not in L1 or L2, `resolve(key)` | returns `None`, local build triggers, `store(key, entry)` populates both L1 and L2 | R-15.11.1 |

### TC-15.11.7.1 Idempotent Upload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload entry with key K, upload same key K again | second upload returns `Ok(())`, no overwrite | R-15.11.7 |

### TC-15.11.7.2 CI Population All Platforms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | CLI populates entries for Windows, macOS, Linux | `exists(key_win) && exists(key_mac) && exists(key_linux)` all true | R-15.11.7 |

### TC-15.11.4.1 Prefetch Parallel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `prefetch_all(100 keys)` with concurrency=8 | all 100 entries in L1, max 8 concurrent downloads observed | R-15.11.4 |

### TC-15.11.6.2 Bandwidth Limit Respected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set `bandwidth_limit = 1_000_000` (1 MB/s), download 10 MB | download takes >= 9 seconds | R-15.11.6 |

### TC-15.11.4.2 First Launch Under 10 Minutes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | New clone with warm L2 cache, run prefetch | all assets available within 10 minutes | R-15.11.4 |

### TC-15.11.1.I3 Integrity Check on Download

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Corrupt bytes in transit (flip bit), download entry | `Err(CacheError::IntegrityError { expected, actual })` | R-15.11.1 |

### TC-15.11.5.I1 GC Scheduled Run

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Configure GC schedule, wait for trigger, check results | `GcResult.bytes_freed > 0` for entries past retention | R-15.11.5 |

### TC-15.11.8.I1 Metrics OTel Export

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record hits/misses, call `export_otel()` | returns valid `Vec<OtelMetric>` with correct counter values | R-15.11.8 |

### TC-15.11.6.I1 Platform HTTP Client

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload and fetch entry on macOS (NSURLSession) | round-trip succeeds, data matches | R-15.11.6 |
| 2 | Upload and fetch entry on Windows (WinHTTP) | round-trip succeeds, data matches | R-15.11.6 |
| 3 | Upload and fetch entry on Linux (libcurl) | round-trip succeeds, data matches | R-15.11.6 |

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
