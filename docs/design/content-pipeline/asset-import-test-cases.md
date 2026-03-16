# Asset Import and Database Test Cases

Companion test cases for [asset-import.md](asset-import.md).

## Unit Tests

### TC-12.3.1.1 Content Hash Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1 MB buffer of 0xAA bytes | BLAKE3 hash H1 | R-12.3.1 |
| 2 | Same 1 MB buffer of 0xAA bytes (second call) | BLAKE3 hash == H1 | R-12.3.1 |

### TC-12.3.1.2 Content Hash Uniqueness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1 MB buffer of 0xAA bytes | BLAKE3 hash H1 | R-12.3.1 |
| 2 | 1 MB buffer of 0xBB bytes | BLAKE3 hash H2 != H1 | R-12.3.1 |

### TC-12.3.1.3 CAS Store and Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store 10 KB blob B1 into CAS | Returns hash H1 | R-12.3.1 |
| 2 | Load CAS by hash H1 | Returns blob == B1 | R-12.3.1 |

### TC-12.3.1.4 CAS Deduplication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store 10 KB blob B1 into CAS | Returns hash H1, blob count = 1 | R-12.3.1 |
| 2 | Store identical 10 KB blob B1 again | Returns hash H1, blob count = 1 | R-12.3.1 |

### TC-12.3.1.5 CAS GC Removes Unreferenced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store blobs B1, B2; reference set = {H1} | Both blobs exist on disk | R-12.3.1 |
| 2 | Run GC with reference set {H1} | B1 exists, B2 removed | R-12.3.1 |

### TC-12.3.1.6 CAS GC Preserves Referenced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store blobs B1, B2; reference set = {H1, H2} | Both blobs exist | R-12.3.1 |
| 2 | Run GC with reference set {H1, H2} | Both blobs still exist | R-12.3.1 |

### TC-12.3.2.1 Metadata Put and Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Put metadata M1 keyed by AssetId A1 | Ok | R-12.3.2 |
| 2 | Get metadata by AssetId A1 | Returns M1 with all fields matching | R-12.3.2 |

### TC-12.3.2.2 Metadata Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Put metadata M1 keyed by AssetId A1 | Ok | R-12.3.2 |
| 2 | Remove metadata by AssetId A1 | Ok | R-12.3.2 |
| 3 | Get metadata by AssetId A1 | Returns None | R-12.3.2 |

### TC-12.1.5.1 Transaction Commit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin transaction, put M1 at A1, put M2 at A2 | Tx handle | R-12.1.5 |
| 2 | Commit transaction | Ok | R-12.1.5 |
| 3 | Get A1 and A2 outside transaction | Returns M1 and M2 | R-12.1.5 |

### TC-12.1.5.2 Transaction Rollback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin transaction, put M1 at A1, put M2 at A2 | Tx handle | R-12.1.5 |
| 2 | Drop transaction without commit | No error | R-12.1.5 |
| 3 | Get A1 and A2 | Returns None for both | R-12.1.5 |

### TC-12.2.8.1 Dependency Graph Add and Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add edges A->B, A->C, B->D | Ok | R-12.2.8 |
| 2 | Query dependents of A | {B, C} | R-12.2.8 |
| 3 | Query dependencies of D | {B} | R-12.2.8 |

### TC-12.3.4.1 Transitive Dependents

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add edges A->B, B->C, C->D | Ok | R-12.3.4 |
| 2 | Query transitive dependents of A | {B, C, D} | R-12.3.4 |

### TC-12.2.8.2 Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add edges A->B, B->C, C->A | CycleDetected error with path [A, B, C, A] | R-12.2.8 |

### TC-12.3.4.2 Topological Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add edges A->B, A->C, B->D, C->D | Ok | R-12.3.4 |
| 2 | Topological sort | A before B and C; B and C before D | R-12.3.4 |

### TC-12.3.3.1 Cache Hit Skips Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset X (populates cache) | Importer called, result R1 | R-12.3.3 |
| 2 | Import asset X again (unchanged) | Importer not called, returns R1 | R-12.3.3 |

### TC-12.3.3.2 Cache Miss Triggers Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset X (not in cache) | Importer called, result stored | R-12.3.3 |

### TC-12.3.3.3 Cache Invalidation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset X (populates cache) | Importer called | R-12.3.3 |
| 2 | Invalidate cache key for X | Ok | R-12.3.3 |
| 3 | Import asset X again | Importer called (cache miss) | R-12.3.3 |

### TC-12.1.4.1 Validate Native Magic Bytes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | File with magic bytes 0xDEADBEEF (wrong) | Error with byte offset 0 | R-12.1.4 |

### TC-12.1.4.2 Validate Native Version

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | File with correct magic, version 0 (unsupported) | Error with fix suggestion "upgrade to v2" | R-12.1.4 |

### TC-12.1.4.3 Validate Native Hash Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | File with correct header but corrupted payload | HashMismatch error | R-12.1.4 |

### TC-12.1.4.4 Validate Texture Dimensions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PNG file 32768x32768 (exceeds max) | Warning "texture exceeds maximum dimensions" | R-12.1.4 |

### TC-12.1.4.5 Validate Audio Sample Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | WAV file with sample rate 7000 Hz | Error "unsupported sample rate 7000" | R-12.1.4 |

### TC-12.3.5.1 Search by Text

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert assets "hero_sword", "hero_shield", "enemy_axe" | Ok | R-12.3.5 |
| 2 | Search text "hero" | Returns {hero_sword, hero_shield} | R-12.3.5 |

### TC-12.3.5.2 Search by Tag

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tag hero_sword with "weapon"; tag hero_shield with "armor" | Ok | R-12.3.5 |
| 2 | Filter by tag "weapon" | Returns {hero_sword} | R-12.3.5 |

### TC-12.3.5.3 Faceted Search

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assets: texture hero_sword (tag=weapon), mesh hero_shield (tag=armor) | Ok | R-12.3.5 |
| 2 | Filter type=texture AND tag=weapon | Returns {hero_sword} | R-12.3.5 |

### TC-12.3.10.1 Version Record and History

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record versions V1, V2, V3 for asset A1 | Ok | R-12.3.10 |
| 2 | List version history for A1 | [V3, V2, V1] (newest first) | R-12.3.10 |

### TC-12.3.10.2 Version Restore

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record V1 (hash H1), V2 (hash H2) for A1 | Ok | R-12.3.10 |
| 2 | Restore A1 to V1 | Content hash == H1 | R-12.3.10 |

### TC-12.1.7.1 Importer Registry Find

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register PngImporter for extension ".png" | Ok | F-12.1.2 |
| 2 | Find importer for ".png" | Returns PngImporter | F-12.1.2 |

### TC-12.1.7.2 Importer Registry Unknown Extension

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Find importer for ".xyz" (not registered) | Returns None | F-12.1.2 |

## Integration Tests

### TC-12.1.1.I1 Native Format End-to-End Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Native-format file with valid header and payload | CAS blob stored, metadata entry created, dep graph edge added | R-12.1.1 |

### TC-12.1.2.I1 PNG End-to-End Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 512x512 RGBA PNG file (sRGB) | CAS blob with decoded sRGB pixel data | R-12.1.2 |

### TC-12.1.2.I2 EXR Linear Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 512x512 EXR file (linear) | CAS blob with linear color space metadata | R-12.1.2 |

### TC-12.1.3.I1 WAV Metadata Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | WAV file: 44100 Hz, stereo, loop point at sample 22050 | Metadata: sample_rate=44100, channels=2, loop_start=22050 | R-12.1.3 |

### TC-12.1.3.I2 FLAC Metadata Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | FLAC file: 48000 Hz, mono | Metadata: sample_rate=48000, channels=1 | R-12.1.3 |

### TC-12.1.3.I3 Ogg Vorbis Metadata Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ogg Vorbis file: 44100 Hz, stereo | Metadata: sample_rate=44100, channels=2 | R-12.1.3 |

### TC-12.1.5.I1 Batch Import 100 Assets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 valid asset files (mixed formats) | All 100 assets imported, 100 metadata entries, progress reports 0-100% | R-12.1.5 |

### TC-12.1.5.I2 Batch Cancel Rollback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin batch import of 100 assets; cancel at 50% | No partial metadata entries; database unchanged | R-12.1.5 |

### TC-12.3.4.I1 Incremental Reimport

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import texture T1 used by materials M1, M2; modify T1 | Only T1, M1, M2 reimported; unrelated assets skipped | R-12.3.4 |

### TC-12.3.4.I2 Incremental vs Full Build Identity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full build of 50 assets; modify 1; incremental rebuild | Incremental output byte-identical to full rebuild | R-12.3.4 |

### TC-12.1.4.I1 Corrupt File Graceful Handling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Truncated file (0 bytes) | Error returned, no crash | R-12.1.4 |
| 2 | Malformed PNG (invalid IHDR) | Error returned, no crash | R-12.1.4 |

### TC-12.3.1.I1 Import Duplicate Deduplication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import same file from path /a/tex.png and /b/tex.png | 1 CAS blob, 2 metadata entries with same hash | R-12.3.1 |

### TC-12.3.3.I1 Cache Across Restarts

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset X, shutdown, restart, import X again | Cache hit on second import; importer not called | R-12.3.3 |

### TC-12.3.10.I1 Version Restore Content

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create versions V1 (hash H1), V2, V3; restore to V1 | Active content hash == H1 | R-12.3.10 |

### TC-12.3.2.I1 Journal Crash Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin transaction, write 5 entries, simulate crash before commit | After restart, journal replay restores clean state; no partial writes | R-12.3.2 |

### TC-12.1.4.I2 CI Validation Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run headless validation on file with schema violation | Non-zero exit code | R-12.1.4 |

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
