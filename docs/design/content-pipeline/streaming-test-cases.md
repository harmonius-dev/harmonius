# Asset Streaming and I/O Test Cases

Companion test cases for [streaming.md](streaming.md).

## Unit Tests

### TC-12.5.1.1 VFS Mount Priority Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mount store A at priority 10, B at 20, C at 30 | Ok | R-12.5.1 |
| 2 | Resolve path "/textures/hero.tex" (exists in A and C) | Returns data from C (highest priority) | R-12.5.1 |

### TC-12.5.1.2 VFS Unmount Removes Paths

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mount store S; resolve "/data/asset.bin" | Returns data | R-12.5.1 |
| 2 | Unmount store S; resolve "/data/asset.bin" | Returns NotFound | R-12.5.1 |

### TC-12.5.1.3 VFS List Deduplicates

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mount store A (has x.tex, y.tex) and B (has x.tex, z.tex) | Ok | R-12.5.1 |
| 2 | List "/textures/" | Returns {x.tex, y.tex, z.tex} (x.tex once only) | R-12.5.1 |

### TC-12.5.6.1 Priority Queue Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue 100 requests with varied priorities | Ok | R-12.5.6 |
| 2 | Call dequeue_batch | Returns in descending composite score order | R-12.5.6 |

### TC-12.5.6.2 Priority Queue Aging

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue Prefetch request R1 at frame 0 | Ok | R-12.5.6 |
| 2 | Enqueue Prefetch request R2 at frame 100; dequeue | R1 ranks above R2 (aged 100 frames) | R-12.5.6 |

### TC-12.5.6.3 Priority Queue Coalesce

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue 10 requests to adjacent archive offsets | Ok | R-12.5.6 |
| 2 | Call coalesce_adjacent | Fewer I/O operations than 10 | R-12.5.6 |

### TC-12.5.6.4 Priority Queue Cancel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue request R1; cancel R1 by ID | Ok | R-12.5.6 |
| 2 | Call dequeue_batch | R1 not in results | R-12.5.6 |

### TC-12.5.8.1 Pak Header Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize PakHeader with version=1, entry_count=100 | Byte buffer | R-12.5.8 |
| 2 | Deserialize byte buffer | All fields match original | R-12.5.8 |

### TC-12.5.8.2 Pak Lookup O(1)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create pak with 10,000 entries; lookup entry #9999 | Lookup time does not scale with entry count | R-12.5.8 |

### TC-12.5.8.3 Pak Directory Integrity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create valid pak; corrupt 1 byte of directory section | open() returns DirectoryCorrupted error | R-12.5.8 |

### TC-12.5.9.1 LZ4 Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 64 KB random data via LZ4; decompress | Byte-for-byte match with original | R-12.5.9 |

### TC-12.5.9.2 Zstd Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 1 MB texture data via Zstd; decompress | Byte-for-byte match with original | R-12.5.9 |

### TC-12.5.9.3 Codec Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | select_codec(AssetType::Audio) | LZ4 | R-12.5.9 |
| 2 | select_codec(AssetType::Script) | LZ4 | R-12.5.9 |
| 3 | select_codec(AssetType::Texture) | Zstd | R-12.5.9 |
| 4 | select_codec(AssetType::Mesh) | Zstd | R-12.5.9 |
| 5 | select_codec(AssetType::Animation) | Zstd | R-12.5.9 |

### TC-12.5.7.1 Budget Pressure Levels

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate to 80% of budget | Pressure level = Warning | R-12.5.7 |
| 2 | Allocate to 90% of budget | Pressure level = Critical | R-12.5.7 |
| 3 | Allocate to 95% of budget | Pressure level = Emergency | R-12.5.7 |

### TC-12.5.7.2 Budget Allocation Denied

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill to Emergency pressure level | Pressure = Emergency | R-12.5.7 |
| 2 | Attempt allocate() for new request | Returns false | R-12.5.7 |

### TC-12.5.4.1 Texture Residency Mask

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Request mips 2-5; mark mip 3 resident | Ok | R-12.5.4 |
| 2 | Query is_resident(3) | true | R-12.5.4 |
| 3 | Query is_resident(2) | false | R-12.5.4 |

### TC-12.5.5.1 Mesh Residency Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mark LODs 0-3 resident; evict above LOD 2 | LODs 0-1 evicted; LODs 2-3 remain resident | R-12.5.5 |

### TC-12.5.10.1 Content Hash Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load asset with tampered content hash | IntegrityError returned | R-12.5.10 |

### TC-12.5.10.2 Download Manager Pause and Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pause downloads; call download() | Returns Paused | R-12.5.10 |
| 2 | Resume downloads; call download() | Download succeeds | R-12.5.10 |

### TC-12.5.2.1 Stream Handle Cancel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit load request; cancel by handle | Handle returns Cancelled | R-12.5.2 |

## Integration Tests

### TC-12.5.1.I1 VFS Loose, Pak, and CDN Mount

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mount loose-file dir, pak archive, mock HTTP CDN; read same asset path from each | Identical content from all three sources | R-12.5.1 |

### TC-12.5.2.I1 Async Read 100 MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 100 MB asset via async I/O on each platform | Completion through IOCP/GCD/io_uring; no std::fs calls | R-12.5.2 |

### TC-12.5.3.I1 GPU Direct Storage 256 MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 256 MB compressed texture pack via GPU direct storage | Pixel values correct; CPU utilization < 5% | R-12.5.3 |

### TC-12.5.4.I1 Texture Streaming 1000

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render 1000 textures; only screen-density mips resident | Mips match screen-space density | R-12.5.4 |
| 2 | Move camera closer | Higher mips arrive within 500 ms | R-12.5.4 |

### TC-12.5.5.I1 Mesh Streaming 10000

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place 10,000 meshes; distant meshes use coarse LODs | Correct LOD per distance | R-12.5.5 |
| 2 | Approach camera | Fine LODs stream in with dithered cross-fade | R-12.5.5 |

### TC-12.5.6.I1 Priority High Before Low

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 100 low-priority and 10 high-priority requests | All 10 high-priority complete before 90% of low-priority | R-12.5.6 |

### TC-12.5.7.I1 Memory Pressure 512 MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set budget to 512 MB; stream until full | Eviction occurs; no crash; usage within 10% of budget | R-12.5.7 |

### TC-12.5.8.I1 Pak Sequential Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Read spatially adjacent assets from pak | Throughput > 90% of raw disk bandwidth | R-12.5.8 |

### TC-12.5.8.I2 Multi-Archive DLC

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mount base + DLC archives; read overlapping asset | DLC version returned | R-12.5.8 |
| 2 | Unmount DLC; read same asset | Base version returned | R-12.5.8 |

### TC-12.5.10.I1 CDN Download and Verify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove asset from local archive; access via VFS | CDN download, hash verified, locally cached | R-12.5.10 |

### TC-12.5.10.I2 CDN Corrupt Retry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serve corrupted download from mock CDN | Hash mismatch detected; download retried | R-12.5.10 |

### TC-12.5.7.I2 Mobile Memory Warning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate iOS didReceiveMemoryWarning | Immediate Emergency eviction of non-visible assets | R-12.5.7 |

### TC-12.5.10.I3 Mobile Metered Pause

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate cellular connection | Downloads auto-pause | R-12.5.10 |
| 2 | Switch to WiFi | Downloads resume | R-12.5.10 |

## Stress Tests

### TC-12.5.4.S1 Flythrough No Pop-Ins

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fly camera through dense open world at max speed for 120 s | Zero missing textures, zero missing meshes, zero hard LOD pops | R-12.5.4 |

### TC-12.5.7.S1 Pressure Retains Visible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Force memory pressure while rendering | Eviction drops only off-screen assets; on-screen remain resident | R-12.5.7 |

### TC-12.5.7.S2 Budget 120s Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream assets continuously for 120 s | Memory usage stays within 10% of budget at all times | R-12.5.7 |

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
