# R-12.5 — Streaming & I/O Requirements

## Virtual File System

### R-12.5.1 Virtual File System

The engine **SHALL** provide a virtual file system that presents a unified path namespace over
multiple backing stores (loose files, pak/archive files, and remote HTTP stores), routing all
asset access through the VFS to decouple content layout from physical storage.

- **Derived from:** [F-12.5.1](../../features/content-pipeline/streaming-io.md)
- **Rationale:** A unified VFS ensures all subsystems access assets through a single abstraction
  regardless of whether the build is a development loose-file layout or a shipping archive.
- **Verification:** Integration test: mount a loose-file directory, a pak archive, and a mock
  HTTP backing store under distinct mount points. Read the same logical asset path from each
  store and verify identical content. Verify that unmounting a store makes its paths unavailable
  without affecting other mounts.

## Async Asset Loading

### R-12.5.2 Platform-Native Async I/O

The engine **SHALL** perform all asset file reads through platform-native async I/O APIs (IOCP
on Windows, Grand Central Dispatch dispatch_io on macOS, io_uring on Linux) with direct I/O
bypassing the CPU page cache, using zero standard library file I/O anywhere in the asset loading
path.

- **Derived from:** [F-12.5.2](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Platform-native async I/O provides the highest throughput and most predictable
  latency by avoiding kernel page cache pollution and user-space thread blocking.
- **Verification:** Integration test per platform: load a 100 MB asset via the async I/O path.
  Verify completion arrives through the platform-native mechanism (IOCP/GCD/io_uring). Verify
  no `std::fs` or `std::io::Read` calls exist in the asset loading path via static analysis.
  Benchmark: verify throughput reaches at least 80% of raw sequential disk bandwidth.

### R-12.5.3 GPU Direct Storage

The engine **SHALL** transfer compressed asset data from SSD directly into GPU memory via
file-to-GPU DMA (DirectStorage on Windows, Metal I/O on macOS), with a compute shader
decompressing the data in place, bypassing CPU involvement for bulk asset transfers.

- **Derived from:** [F-12.5.3](../../features/content-pipeline/streaming-io.md)
- **Rationale:** GPU direct storage saturates NVMe bandwidth for open-world streaming by
  eliminating CPU-side copies and decompression bottlenecks.
- **Verification:** Integration test: load a 256 MB compressed texture pack via GPU direct
  storage. Verify the data arrives in GPU memory and decompresses correctly by sampling pixel
  values. Benchmark: verify CPU utilization during the transfer is under 5% and throughput
  exceeds 3 GB/s on NVMe hardware. On Linux, verify the io_uring fallback with CPU-side
  staging produces correct results.

## Streaming LOD

### R-12.5.4 Texture Streaming

The engine **SHALL** stream texture mip levels on demand based on screen-space texel density,
with a residency manager that tracks resident mips, schedules async loads for requested mips,
and evicts low-priority mips under memory pressure, using sparse binding for per-page virtual
texture mapping.

- **Derived from:** [F-12.5.4](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Loading only the mip levels required at the current viewing distance minimizes
  GPU memory consumption while maintaining visual quality at screen resolution.
- **Verification:** Integration test: render a scene with 1,000 textures. Verify that only mip
  levels matching the current screen-space density are resident. Move the camera closer and
  verify higher mips stream in within 500 ms. Verify evicted mips free GPU memory. Benchmark:
  verify total GPU memory usage stays within the configured texture budget.

### R-12.5.5 Mesh Streaming

The engine **SHALL** stream mesh LOD levels and meshlet groups on demand based on screen-space
projected size and camera velocity, keeping coarse LODs permanently resident and loading fine
LODs asynchronously as the camera approaches, with dithered cross-fade transitions to prevent
LOD popping.

- **Derived from:** [F-12.5.5](../../features/content-pipeline/streaming-io.md)
- **Rationale:** On-demand mesh streaming enables open worlds with millions of unique meshes
  without requiring all geometry to be resident in GPU memory simultaneously.
- **Verification:** Integration test: place 10,000 meshes in a scene. Verify that distant meshes
  use coarse LODs and fine LODs load as the camera approaches. Capture frames during LOD
  transitions and verify dithered cross-fade is active (no hard pops). Benchmark: verify GPU
  memory usage stays within the configured mesh budget across a 60-second camera fly-through.

## Priority and Memory Management

### R-12.5.6 Streaming Priority Queues

The engine **SHALL** schedule all pending I/O requests through a priority queue ordered by
screen-space size, camera distance, asset type weight, and frame deadline, coalescing adjacent
requests and aging stale requests to prevent priority inversion.

- **Derived from:** [F-12.5.6](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Priority-based scheduling ensures frame-critical assets load before background
  prefetch, preventing visual pop-in for on-screen content.
- **Verification:** Integration test: submit 100 low-priority and 10 high-priority I/O requests
  simultaneously. Verify all high-priority requests complete before at least 90% of low-priority
  requests. Verify a stale low-priority request eventually promotes above newer low-priority
  requests after aging. Benchmark: verify request coalescing reduces total I/O operations by at
  least 20% for spatially adjacent assets.

### R-12.5.7 Memory Pressure Response

The engine **SHALL** monitor GPU and CPU memory usage against configurable thresholds and
progressively evict lowest-priority streamed assets, reduce streaming quality targets, and signal
subsystems to release cached data when memory pressure exceeds the configured budget.

- **Derived from:** [F-12.5.7](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Active memory pressure management prevents out-of-memory crashes during
  open-world traversal by gracefully degrading quality before hard limits are reached.
- **Verification:** Integration test: configure a memory budget of 512 MB. Stream assets until
  usage reaches the budget. Verify the system evicts low-priority assets and reduces quality
  targets without crashing. Verify memory usage stays within 10% of the configured budget over
  a 120-second stress test. Verify subsystem release signals fire when thresholds are crossed.

## Archive and Compression

### R-12.5.8 Pak / Archive Files

The engine **SHALL** pack processed assets into seekable archive files with a central directory
providing O(1) lookup by asset ID, organized by streaming region and priority tier for spatial
locality, and support mounting multiple archives simultaneously for expansion packs and DLC.

- **Derived from:** [F-12.5.8](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Seekable archives with spatial organization maximize sequential read throughput
  for open-world streaming and eliminate per-file system call overhead.
- **Verification:** Integration test: pack 10,000 assets into an archive organized by streaming
  region. Verify O(1) lookup by asset ID (constant-time regardless of archive size). Mount two
  archives simultaneously and verify assets from both are accessible. Benchmark: verify
  sequential read throughput of spatially adjacent assets exceeds 90% of raw disk bandwidth.

### R-12.5.9 Compression (LZ4, Zstd)

The engine **SHALL** compress archive chunks using LZ4 for latency-sensitive assets (audio, UI)
and Zstd for throughput-sensitive assets (textures, meshes), with chunk boundaries aligned to
streaming granularity so individual assets decompress independently.

- **Derived from:** [F-12.5.9](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Per-chunk compression with codec selection by asset type balances decompression
  latency against compression ratio for each workload.
- **Verification:** Integration test: compress and decompress assets of each type and verify data
  integrity via content hashes. Verify audio/UI assets use LZ4 and texture/mesh assets use Zstd.
  Verify a single chunk can be decompressed without reading adjacent chunks. Benchmark: verify
  LZ4 decompression latency is under 1 ms for a 64 KB chunk and Zstd achieves at least 3:1
  compression ratio for texture data.

## Download-on-Demand

### R-12.5.10 Download-on-Demand Patching

The engine **SHALL** stream assets from a remote CDN on first access when not present in local
archives, using a manifest that maps asset IDs to CDN URLs and content hashes (using BLAKE3,
consistent with R-12.1.1 and R-14.6.6), verifying and decompressing downloaded chunks before
writing them into the local archive for future access.

- **Derived from:** [F-12.5.10](../../features/content-pipeline/streaming-io.md)
- **Rationale:** Download-on-demand enables players to begin playing immediately while content
  streams in the background, reducing initial install size.
- **Verification:** Integration test: remove an asset from the local archive. Access it through
  the VFS and verify it downloads from the mock CDN, passes hash verification, and becomes
  available locally for subsequent access without re-downloading. Verify a corrupted download
  (hash mismatch) is rejected and retried. Benchmark: verify download throughput saturates the
  available network bandwidth for sequential asset fetches.
