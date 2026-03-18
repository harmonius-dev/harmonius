# Asset Streaming and I/O Test Cases

Companion test cases for [streaming.md](streaming.md).

## Unit Tests

### TC-12.5.1.1 VFS Mount Priority Order

| # | Requirement |
|---|-------------|
| 1 | R-12.5.1    |
| 2 | R-12.5.1    |

1. **#1** — Mount store A at priority 10, B at 20, C at 30
   - **Expected:** Ok
2. **#2** — Resolve path "/textures/hero.tex" (exists in A and C)
   - **Expected:** Returns data from C (highest priority)

### TC-12.5.1.2 VFS Unmount Removes Paths

| # | Requirement |
|---|-------------|
| 1 | R-12.5.1    |
| 2 | R-12.5.1    |

1. **#1** — Mount store S; resolve "/data/asset.bin"
   - **Expected:** Returns data
2. **#2** — Unmount store S; resolve "/data/asset.bin"
   - **Expected:** Returns NotFound

### TC-12.5.1.3 VFS List Deduplicates

| # | Requirement |
|---|-------------|
| 1 | R-12.5.1    |
| 2 | R-12.5.1    |

1. **#1** — Mount store A (has x.tex, y.tex) and B (has x.tex, z.tex)
   - **Expected:** Ok
2. **#2** — List "/textures/"
   - **Expected:** Returns {x.tex, y.tex, z.tex} (x.tex once only)

### TC-12.5.6.1 Priority Queue Ordering

| # | Requirement |
|---|-------------|
| 1 | R-12.5.6    |
| 2 | R-12.5.6    |

1. **#1** — Enqueue 100 requests with varied priorities
   - **Expected:** Ok
2. **#2** — Call dequeue_batch
   - **Expected:** Returns in descending composite score order

### TC-12.5.6.2 Priority Queue Aging

| # | Requirement |
|---|-------------|
| 1 | R-12.5.6    |
| 2 | R-12.5.6    |

1. **#1** — Enqueue Prefetch request R1 at frame 0
   - **Expected:** Ok
2. **#2** — Enqueue Prefetch request R2 at frame 100; dequeue
   - **Expected:** R1 ranks above R2 (aged 100 frames)

### TC-12.5.6.3 Priority Queue Coalesce

| # | Requirement |
|---|-------------|
| 1 | R-12.5.6    |
| 2 | R-12.5.6    |

1. **#1** — Enqueue 10 requests to adjacent archive offsets
   - **Expected:** Ok
2. **#2** — Call coalesce_adjacent
   - **Expected:** Fewer I/O operations than 10

### TC-12.5.6.4 Priority Queue Cancel

| # | Requirement |
|---|-------------|
| 1 | R-12.5.6    |
| 2 | R-12.5.6    |

1. **#1** — Enqueue request R1; cancel R1 by ID
   - **Expected:** Ok
2. **#2** — Call dequeue_batch
   - **Expected:** R1 not in results

### TC-12.5.8.1 Pak Header Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.5.8    |
| 2 | R-12.5.8    |

1. **#1** — Serialize PakHeader with version=1, entry_count=100
   - **Expected:** Byte buffer
2. **#2** — Deserialize byte buffer
   - **Expected:** All fields match original

### TC-12.5.8.2 Pak Lookup O(1)

| # | Requirement |
|---|-------------|
| 1 | R-12.5.8    |

1. **#1** — Create pak with 10,000 entries; lookup entry #9999
   - **Expected:** Lookup time does not scale with entry count

### TC-12.5.8.3 Pak Directory Integrity

| # | Requirement |
|---|-------------|
| 1 | R-12.5.8    |

1. **#1** — Create valid pak; corrupt 1 byte of directory section
   - **Expected:** open() returns DirectoryCorrupted error

### TC-12.5.9.1 LZ4 Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.5.9    |

1. **#1** — Compress 64 KB random data via LZ4; decompress
   - **Expected:** Byte-for-byte match with original

### TC-12.5.9.2 Zstd Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.5.9    |

1. **#1** — Compress 1 MB texture data via Zstd; decompress
   - **Expected:** Byte-for-byte match with original

### TC-12.5.9.3 Codec Selection

| # | Requirement |
|---|-------------|
| 1 | R-12.5.9    |
| 2 | R-12.5.9    |
| 3 | R-12.5.9    |
| 4 | R-12.5.9    |
| 5 | R-12.5.9    |

1. **#1** — select_codec(AssetType::Audio)
   - **Expected:** LZ4
2. **#2** — select_codec(AssetType::Script)
   - **Expected:** LZ4
3. **#3** — select_codec(AssetType::Texture)
   - **Expected:** Zstd
4. **#4** — select_codec(AssetType::Mesh)
   - **Expected:** Zstd
5. **#5** — select_codec(AssetType::Animation)
   - **Expected:** Zstd

### TC-12.5.7.1 Budget Pressure Levels

| # | Requirement |
|---|-------------|
| 1 | R-12.5.7    |
| 2 | R-12.5.7    |
| 3 | R-12.5.7    |

1. **#1** — Allocate to 80% of budget
   - **Expected:** Pressure level = Warning
2. **#2** — Allocate to 90% of budget
   - **Expected:** Pressure level = Critical
3. **#3** — Allocate to 95% of budget
   - **Expected:** Pressure level = Emergency

### TC-12.5.7.2 Budget Allocation Denied

| # | Requirement |
|---|-------------|
| 1 | R-12.5.7    |
| 2 | R-12.5.7    |

1. **#1** — Fill to Emergency pressure level
   - **Expected:** Pressure = Emergency
2. **#2** — Attempt allocate() for new request
   - **Expected:** Returns false

### TC-12.5.4.1 Texture Residency Mask

| # | Requirement |
|---|-------------|
| 1 | R-12.5.4    |
| 2 | R-12.5.4    |
| 3 | R-12.5.4    |

1. **#1** — Request mips 2-5; mark mip 3 resident
   - **Expected:** Ok
2. **#2** — Query is_resident(3)
   - **Expected:** true
3. **#3** — Query is_resident(2)
   - **Expected:** false

### TC-12.5.5.1 Mesh Residency Eviction

| # | Requirement |
|---|-------------|
| 1 | R-12.5.5    |

1. **#1** — Mark LODs 0-3 resident; evict above LOD 2
   - **Expected:** LODs 0-1 evicted; LODs 2-3 remain resident

### TC-12.5.10.1 Content Hash Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-12.5.10   |

1. **#1** — Load asset with tampered content hash
   - **Expected:** IntegrityError returned

### TC-12.5.10.2 Download Manager Pause and Resume

| # | Requirement |
|---|-------------|
| 1 | R-12.5.10   |
| 2 | R-12.5.10   |

1. **#1** — Pause downloads; call download()
   - **Expected:** Returns Paused
2. **#2** — Resume downloads; call download()
   - **Expected:** Download succeeds

### TC-12.5.2.1 Stream Handle Cancel

| # | Requirement |
|---|-------------|
| 1 | R-12.5.2    |

1. **#1** — Submit load request; cancel by handle
   - **Expected:** Handle returns Cancelled

## Integration Tests

### TC-12.5.1.I1 VFS Loose, Pak, and CDN Mount

| # | Requirement |
|---|-------------|
| 1 | R-12.5.1    |

1. **#1** — Mount loose-file dir, pak archive, mock HTTP CDN; read same asset path from each
   - **Expected:** Identical content from all three sources

### TC-12.5.2.I1 Async Read 100 MB

| # | Requirement |
|---|-------------|
| 1 | R-12.5.2    |

1. **#1** — Load 100 MB asset via async I/O on each platform
   - **Expected:** Completion through IOCP/GCD/io_uring; no std::fs calls

### TC-12.5.3.I1 GPU Direct Storage 256 MB

| # | Requirement |
|---|-------------|
| 1 | R-12.5.3    |

1. **#1** — Load 256 MB compressed texture pack via GPU direct storage
   - **Expected:** Pixel values correct; CPU utilization < 5%

### TC-12.5.4.I1 Texture Streaming 1000

| # | Requirement |
|---|-------------|
| 1 | R-12.5.4    |
| 2 | R-12.5.4    |

1. **#1** — Render 1000 textures; only screen-density mips resident
   - **Expected:** Mips match screen-space density
2. **#2** — Move camera closer
   - **Expected:** Higher mips arrive within 500 ms

### TC-12.5.5.I1 Mesh Streaming 10000

| # | Requirement |
|---|-------------|
| 1 | R-12.5.5    |
| 2 | R-12.5.5    |

1. **#1** — Place 10,000 meshes; distant meshes use coarse LODs
   - **Expected:** Correct LOD per distance
2. **#2** — Approach camera
   - **Expected:** Fine LODs stream in with dithered cross-fade

### TC-12.5.6.I1 Priority High Before Low

| # | Requirement |
|---|-------------|
| 1 | R-12.5.6    |

1. **#1** — Submit 100 low-priority and 10 high-priority requests
   - **Expected:** All 10 high-priority complete before 90% of low-priority

### TC-12.5.7.I1 Memory Pressure 512 MB

| # | Requirement |
|---|-------------|
| 1 | R-12.5.7    |

1. **#1** — Set budget to 512 MB; stream until full
   - **Expected:** Eviction occurs; no crash; usage within 10% of budget

### TC-12.5.8.I1 Pak Sequential Throughput

| # | Requirement |
|---|-------------|
| 1 | R-12.5.8    |

1. **#1** — Read spatially adjacent assets from pak
   - **Expected:** Throughput > 90% of raw disk bandwidth

### TC-12.5.8.I2 Multi-Archive DLC

| # | Requirement |
|---|-------------|
| 1 | R-12.5.8    |
| 2 | R-12.5.8    |

1. **#1** — Mount base + DLC archives; read overlapping asset
   - **Expected:** DLC version returned
2. **#2** — Unmount DLC; read same asset
   - **Expected:** Base version returned

### TC-12.5.10.I1 CDN Download and Verify

| # | Requirement |
|---|-------------|
| 1 | R-12.5.10   |

1. **#1** — Remove asset from local archive; access via VFS
   - **Expected:** CDN download, hash verified, locally cached

### TC-12.5.10.I2 CDN Corrupt Retry

| # | Requirement |
|---|-------------|
| 1 | R-12.5.10   |

1. **#1** — Serve corrupted download from mock CDN
   - **Expected:** Hash mismatch detected; download retried

### TC-12.5.7.I2 Mobile Memory Warning

| # | Requirement |
|---|-------------|
| 1 | R-12.5.7    |

1. **#1** — Simulate iOS didReceiveMemoryWarning
   - **Expected:** Immediate Emergency eviction of non-visible assets

### TC-12.5.10.I3 Mobile Metered Pause

| # | Requirement |
|---|-------------|
| 1 | R-12.5.10   |
| 2 | R-12.5.10   |

1. **#1** — Simulate cellular connection
   - **Expected:** Downloads auto-pause
2. **#2** — Switch to WiFi
   - **Expected:** Downloads resume

## Stress Tests

### TC-12.5.4.S1 Flythrough No Pop-Ins

| # | Requirement |
|---|-------------|
| 1 | R-12.5.4    |

1. **#1** — Fly camera through dense open world at max speed for 120 s
   - **Expected:** Zero missing textures, zero missing meshes, zero hard LOD pops

### TC-12.5.7.S1 Pressure Retains Visible

| # | Requirement |
|---|-------------|
| 1 | R-12.5.7    |

1. **#1** — Force memory pressure while rendering
   - **Expected:** Eviction drops only off-screen assets; on-screen remain resident

### TC-12.5.7.S2 Budget 120s Stability

| # | Requirement |
|---|-------------|
| 1 | R-12.5.7    |

1. **#1** — Stream assets continuously for 120 s
   - **Expected:** Memory usage stays within 10% of budget at all times

## Benchmarks

### TC-12.5.2.B1 Async I/O Throughput NVMe

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Async read on NVMe SSD | Throughput | >= 80% raw bandwidth | R-12.5.2 |

### TC-12.5.2.B2 Async I/O Throughput HDD

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Async read on HDD | Throughput | >= 80% raw bandwidth | R-12.5.2 |

### TC-12.5.3.B1 GPU Direct Storage Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | GPU direct storage on NVMe | Throughput | >= 3 GB/s | R-12.5.3 |

### TC-12.5.3.B2 GPU DMA CPU Utilization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | GPU DMA transfer of 256 MB | CPU utilization | < 5% | R-12.5.3 |

### TC-12.5.9.B1 LZ4 Decompress

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Decompress 64 KB LZ4 chunk | Wall time | < 1 ms | R-12.5.9 |

### TC-12.5.9.B2 Zstd Compression Ratio

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compress texture data with Zstd | Compression ratio | >= 3:1 | R-12.5.9 |

### TC-12.5.8.B1 Pak Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Lookup in pak with 10K entries | Wall time | < 1 us | R-12.5.8 |

### TC-12.5.6.B1 Priority Queue Dequeue

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dequeue batch from 1K-request queue | Wall time | < 100 us | R-12.5.6 |

### TC-12.5.6.B2 Request Coalescing Reduction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Coalesce adjacent I/O requests | I/O reduction | >= 20% | R-12.5.6 |

### TC-12.5.4.B1 Texture Mip Stream-In Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Request higher mip for visible texture | Wall time | < 500 ms | R-12.5.4 |
