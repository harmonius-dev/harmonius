# Asset Import and Database Test Cases

Companion test cases for [asset-import.md](asset-import.md).

## Unit Tests

### TC-12.3.1.1 Content Hash Determinism

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |
| 2 | R-12.3.1    |

1. **#1** — 1 MB buffer of 0xAA bytes
   - **Expected:** BLAKE3 hash H1
2. **#2** — Same 1 MB buffer of 0xAA bytes (second call)
   - **Expected:** BLAKE3 hash == H1

### TC-12.3.1.2 Content Hash Uniqueness

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |
| 2 | R-12.3.1    |

1. **#1** — 1 MB buffer of 0xAA bytes
   - **Expected:** BLAKE3 hash H1
2. **#2** — 1 MB buffer of 0xBB bytes
   - **Expected:** BLAKE3 hash H2 != H1

### TC-12.3.1.3 CAS Store and Load

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |
| 2 | R-12.3.1    |

1. **#1** — Store 10 KB blob B1 into CAS
   - **Expected:** Returns hash H1
2. **#2** — Load CAS by hash H1
   - **Expected:** Returns blob == B1

### TC-12.3.1.4 CAS Deduplication

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |
| 2 | R-12.3.1    |

1. **#1** — Store 10 KB blob B1 into CAS
   - **Expected:** Returns hash H1, blob count = 1
2. **#2** — Store identical 10 KB blob B1 again
   - **Expected:** Returns hash H1, blob count = 1

### TC-12.3.1.5 CAS GC Removes Unreferenced

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |
| 2 | R-12.3.1    |

1. **#1** — Store blobs B1, B2; reference set = {H1}
   - **Expected:** Both blobs exist on disk
2. **#2** — Run GC with reference set {H1}
   - **Expected:** B1 exists, B2 removed

### TC-12.3.1.6 CAS GC Preserves Referenced

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |
| 2 | R-12.3.1    |

1. **#1** — Store blobs B1, B2; reference set = {H1, H2}
   - **Expected:** Both blobs exist
2. **#2** — Run GC with reference set {H1, H2}
   - **Expected:** Both blobs still exist

### TC-12.3.2.1 Metadata Put and Get

| # | Requirement |
|---|-------------|
| 1 | R-12.3.2    |
| 2 | R-12.3.2    |

1. **#1** — Put metadata M1 keyed by AssetId A1
   - **Expected:** Ok
2. **#2** — Get metadata by AssetId A1
   - **Expected:** Returns M1 with all fields matching

### TC-12.3.2.2 Metadata Remove

| # | Requirement |
|---|-------------|
| 1 | R-12.3.2    |
| 2 | R-12.3.2    |
| 3 | R-12.3.2    |

1. **#1** — Put metadata M1 keyed by AssetId A1
   - **Expected:** Ok
2. **#2** — Remove metadata by AssetId A1
   - **Expected:** Ok
3. **#3** — Get metadata by AssetId A1
   - **Expected:** Returns None

### TC-12.1.5.1 Transaction Commit

| # | Requirement |
|---|-------------|
| 1 | R-12.1.5    |
| 2 | R-12.1.5    |
| 3 | R-12.1.5    |

1. **#1** — Begin transaction, put M1 at A1, put M2 at A2
   - **Expected:** Tx handle
2. **#2** — Commit transaction
   - **Expected:** Ok
3. **#3** — Get A1 and A2 outside transaction
   - **Expected:** Returns M1 and M2

### TC-12.1.5.2 Transaction Rollback

| # | Requirement |
|---|-------------|
| 1 | R-12.1.5    |
| 2 | R-12.1.5    |
| 3 | R-12.1.5    |

1. **#1** — Begin transaction, put M1 at A1, put M2 at A2
   - **Expected:** Tx handle
2. **#2** — Drop transaction without commit
   - **Expected:** No error
3. **#3** — Get A1 and A2
   - **Expected:** Returns None for both

### TC-12.2.8.1 Dependency Graph Add and Query

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |
| 2 | R-12.2.8    |
| 3 | R-12.2.8    |

1. **#1** — Add edges A->B, A->C, B->D
   - **Expected:** Ok
2. **#2** — Query dependents of A
   - **Expected:** {B, C}
3. **#3** — Query dependencies of D
   - **Expected:** {B}

### TC-12.3.4.1 Transitive Dependents

| # | Requirement |
|---|-------------|
| 1 | R-12.3.4    |
| 2 | R-12.3.4    |

1. **#1** — Add edges A->B, B->C, C->D
   - **Expected:** Ok
2. **#2** — Query transitive dependents of A
   - **Expected:** {B, C, D}

### TC-12.2.8.2 Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Add edges A->B, B->C, C->A
   - **Expected:** CycleDetected error with path [A, B, C, A]

### TC-12.3.4.2 Topological Order

| # | Requirement |
|---|-------------|
| 1 | R-12.3.4    |
| 2 | R-12.3.4    |

1. **#1** — Add edges A->B, A->C, B->D, C->D
   - **Expected:** Ok
2. **#2** — Topological sort
   - **Expected:** A before B and C; B and C before D

### TC-12.3.3.1 Cache Hit Skips Import

| # | Requirement |
|---|-------------|
| 1 | R-12.3.3    |
| 2 | R-12.3.3    |

1. **#1** — Import asset X (populates cache)
   - **Expected:** Importer called, result R1
2. **#2** — Import asset X again (unchanged)
   - **Expected:** Importer not called, returns R1

### TC-12.3.3.2 Cache Miss Triggers Import

| # | Requirement |
|---|-------------|
| 1 | R-12.3.3    |

1. **#1** — Import asset X (not in cache)
   - **Expected:** Importer called, result stored

### TC-12.3.3.3 Cache Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-12.3.3    |
| 2 | R-12.3.3    |
| 3 | R-12.3.3    |

1. **#1** — Import asset X (populates cache)
   - **Expected:** Importer called
2. **#2** — Invalidate cache key for X
   - **Expected:** Ok
3. **#3** — Import asset X again
   - **Expected:** Importer called (cache miss)

### TC-12.1.4.1 Validate Native Magic Bytes

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |

1. **#1** — File with magic bytes 0xDEADBEEF (wrong)
   - **Expected:** Error with byte offset 0

### TC-12.1.4.2 Validate Native Version

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |

1. **#1** — File with correct magic, version 0 (unsupported)
   - **Expected:** Error with fix suggestion "upgrade to v2"

### TC-12.1.4.3 Validate Native Hash Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |

1. **#1** — File with correct header but corrupted payload
   - **Expected:** HashMismatch error

### TC-12.1.4.4 Validate Texture Dimensions

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |

1. **#1** — PNG file 32768x32768 (exceeds max)
   - **Expected:** Warning "texture exceeds maximum dimensions"

### TC-12.1.4.5 Validate Audio Sample Rate

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |

1. **#1** — WAV file with sample rate 7000 Hz
   - **Expected:** Error "unsupported sample rate 7000"

### TC-12.3.5.1 Search by Text

| # | Requirement |
|---|-------------|
| 1 | R-12.3.5    |
| 2 | R-12.3.5    |

1. **#1** — Insert assets "hero_sword", "hero_shield", "enemy_axe"
   - **Expected:** Ok
2. **#2** — Search text "hero"
   - **Expected:** Returns {hero_sword, hero_shield}

### TC-12.3.5.2 Search by Tag

| # | Requirement |
|---|-------------|
| 1 | R-12.3.5    |
| 2 | R-12.3.5    |

1. **#1** — Tag hero_sword with "weapon"; tag hero_shield with "armor"
   - **Expected:** Ok
2. **#2** — Filter by tag "weapon"
   - **Expected:** Returns {hero_sword}

### TC-12.3.5.3 Faceted Search

| # | Requirement |
|---|-------------|
| 1 | R-12.3.5    |
| 2 | R-12.3.5    |

1. **#1** — Assets: texture hero_sword (tag=weapon), mesh hero_shield (tag=armor)
   - **Expected:** Ok
2. **#2** — Filter type=texture AND tag=weapon
   - **Expected:** Returns {hero_sword}

### TC-12.3.10.1 Version Record and History

| # | Requirement |
|---|-------------|
| 1 | R-12.3.10   |
| 2 | R-12.3.10   |

1. **#1** — Record versions V1, V2, V3 for asset A1
   - **Expected:** Ok
2. **#2** — List version history for A1
   - **Expected:** [V3, V2, V1] (newest first)

### TC-12.3.10.2 Version Restore

| # | Requirement |
|---|-------------|
| 1 | R-12.3.10   |
| 2 | R-12.3.10   |

1. **#1** — Record V1 (hash H1), V2 (hash H2) for A1
   - **Expected:** Ok
2. **#2** — Restore A1 to V1
   - **Expected:** Content hash == H1

### TC-12.1.7.1 Importer Registry Find

| # | Requirement |
|---|-------------|
| 1 | F-12.1.2    |
| 2 | F-12.1.2    |

1. **#1** — Register PngImporter for extension ".png"
   - **Expected:** Ok
2. **#2** — Find importer for ".png"
   - **Expected:** Returns PngImporter

### TC-12.1.7.2 Importer Registry Unknown Extension

| # | Requirement |
|---|-------------|
| 1 | F-12.1.2    |

1. **#1** — Find importer for ".xyz" (not registered)
   - **Expected:** Returns None

## Integration Tests

### TC-12.1.1.I1 Native Format End-to-End Import

| # | Requirement |
|---|-------------|
| 1 | R-12.1.1    |

1. **#1** — Native-format file with valid header and payload
   - **Expected:** CAS blob stored, metadata entry created, dep graph edge added

### TC-12.1.2.I1 PNG End-to-End Import

| # | Requirement |
|---|-------------|
| 1 | R-12.1.2    |

1. **#1** — 512x512 RGBA PNG file (sRGB)
   - **Expected:** CAS blob with decoded sRGB pixel data

### TC-12.1.2.I2 EXR Linear Import

| # | Requirement |
|---|-------------|
| 1 | R-12.1.2    |

1. **#1** — 512x512 EXR file (linear)
   - **Expected:** CAS blob with linear color space metadata

### TC-12.1.3.I1 WAV Metadata Import

| # | Requirement |
|---|-------------|
| 1 | R-12.1.3    |

1. **#1** — WAV file: 44100 Hz, stereo, loop point at sample 22050
   - **Expected:** Metadata: sample_rate=44100, channels=2, loop_start=22050

### TC-12.1.3.I2 FLAC Metadata Import

| # | Requirement |
|---|-------------|
| 1 | R-12.1.3    |

1. **#1** — FLAC file: 48000 Hz, mono
   - **Expected:** Metadata: sample_rate=48000, channels=1

### TC-12.1.3.I3 Ogg Vorbis Metadata Import

| # | Requirement |
|---|-------------|
| 1 | R-12.1.3    |

1. **#1** — Ogg Vorbis file: 44100 Hz, stereo
   - **Expected:** Metadata: sample_rate=44100, channels=2

### TC-12.1.5.I1 Batch Import 100 Assets

| # | Requirement |
|---|-------------|
| 1 | R-12.1.5    |

1. **#1** — 100 valid asset files (mixed formats)
   - **Expected:** All 100 assets imported, 100 metadata entries, progress reports 0-100%

### TC-12.1.5.I2 Batch Cancel Rollback

| # | Requirement |
|---|-------------|
| 1 | R-12.1.5    |

1. **#1** — Begin batch import of 100 assets; cancel at 50%
   - **Expected:** No partial metadata entries; database unchanged

### TC-12.3.4.I1 Incremental Reimport

| # | Requirement |
|---|-------------|
| 1 | R-12.3.4    |

1. **#1** — Import texture T1 used by materials M1, M2; modify T1
   - **Expected:** Only T1, M1, M2 reimported; unrelated assets skipped

### TC-12.3.4.I2 Incremental vs Full Build Identity

| # | Requirement |
|---|-------------|
| 1 | R-12.3.4    |

1. **#1** — Full build of 50 assets; modify 1; incremental rebuild
   - **Expected:** Incremental output byte-identical to full rebuild

### TC-12.1.4.I1 Corrupt File Graceful Handling

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |
| 2 | R-12.1.4    |

1. **#1** — Truncated file (0 bytes)
   - **Expected:** Error returned, no crash
2. **#2** — Malformed PNG (invalid IHDR)
   - **Expected:** Error returned, no crash

### TC-12.3.1.I1 Import Duplicate Deduplication

| # | Requirement |
|---|-------------|
| 1 | R-12.3.1    |

1. **#1** — Import same file from path /a/tex.png and /b/tex.png
   - **Expected:** 1 CAS blob, 2 metadata entries with same hash

### TC-12.3.3.I1 Cache Across Restarts

| # | Requirement |
|---|-------------|
| 1 | R-12.3.3    |

1. **#1** — Import asset X, shutdown, restart, import X again
   - **Expected:** Cache hit on second import; importer not called

### TC-12.3.10.I1 Version Restore Content

| # | Requirement |
|---|-------------|
| 1 | R-12.3.10   |

1. **#1** — Create versions V1 (hash H1), V2, V3; restore to V1
   - **Expected:** Active content hash == H1

### TC-12.3.2.I1 Journal Crash Recovery

| # | Requirement |
|---|-------------|
| 1 | R-12.3.2    |

1. **#1** — Begin transaction, write 5 entries, simulate crash before commit
   - **Expected:** After restart, journal replay restores clean state; no partial writes

### TC-12.1.4.I2 CI Validation Reject

| # | Requirement |
|---|-------------|
| 1 | R-12.1.4    |

1. **#1** — Run headless validation on file with schema violation
   - **Expected:** Non-zero exit code

## Benchmarks

### TC-12.3.1.B1 BLAKE3 Hash Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hash 1 GB file, single core | Wall time | < 1.5 s | R-12.3.1 |

### TC-12.3.1.B2 CAS Store Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Store 10 MB blob via async I/O | Wall time | < 20 ms | R-12.3.1 |

### TC-12.3.1.B3 CAS Dedup Check Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Check dedup for existing 10 MB blob | Wall time | < 1 ms | R-12.3.1 |

### TC-12.1.5.B1 Batch Import Native Assets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Import 100 native assets on 8 cores | Wall time | < 10 s | R-12.1.5 |

### TC-12.1.5.B2 Batch Import Textures

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Import 1000 textures on 8 cores | Wall time | < 60 s | R-12.1.5 |

### TC-12.3.2.B1 Metadata Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Query by AssetId in 1M-entry database | Wall time | < 100 ms | R-12.3.2 |

### TC-12.3.5.B1 Full-Text Search

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full-text search in 1M-entry database | Wall time | < 100 ms | R-12.3.5 |

### TC-12.3.4.B1 Dependency Invalidation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Invalidate and propagate in 10K-asset graph | Wall time | < 50 ms | R-12.3.4 |

### TC-12.3.3.B1 Cache Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single cache key lookup | Wall time | < 0.1 ms | R-12.3.3 |
