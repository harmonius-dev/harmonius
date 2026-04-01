# R-12.5 Streaming & I/O

## Virtual File System

1. **R-12.5.1** — The engine **SHALL** provide a virtual file system that presents a unified path
   namespace over multiple backing stores (loose files, pak/archive files, and remote HTTP stores),
   routing all asset access through the VFS to decouple content layout from physical storage.
   - **Rationale:** A unified VFS ensures all subsystems access assets through a single abstraction
     regardless of whether the build is development or shipping.
   - **Verification:** Mount a loose-file directory, a pak archive, and a mock HTTP store; read the
     same logical path from each and verify identical content; unmount a store and verify its paths
     become unavailable.

## Async Asset Loading

2. **R-12.5.2** — The engine **SHALL** perform all asset file reads through Tokio async I/O with
   direct I/O bypassing the CPU page cache, using zero standard library file I/O in the asset
   loading path. Builds on the core async I/O abstraction defined in F-1.8.1.
   - **Rationale:** Tokio async I/O provides the highest throughput and most predictable latency by
     avoiding kernel page cache pollution and thread blocking.
   - **Verification:** Load a 100 MB asset via async I/O per platform; verify completion via Tokio;
     verify no std::fs calls via static analysis; benchmark at 80%+ of raw sequential disk
     bandwidth.
3. **R-12.5.3** — The engine **SHALL** transfer compressed asset data from SSD directly into GPU
   memory via file-to-GPU DMA (DirectStorage on Windows, Metal I/O on macOS), with a compute shader
   decompressing in place, bypassing CPU for bulk transfers.
   - **Rationale:** GPU direct storage saturates NVMe bandwidth for open-world streaming by
     eliminating CPU-side copies.
   - **Verification:** Load a 256 MB compressed texture pack via GPU direct storage; verify correct
     decompression by sampling pixels; benchmark CPU utilization under 5% and throughput exceeding 3
     GB/s on NVMe.

## Texture and Mesh Streaming

4. **R-12.5.4** — The engine **SHALL** stream texture mip levels on demand based on screen-space
   texel density, with a residency manager that tracks resident mips, schedules async loads, and
   evicts low-priority mips under memory pressure, using sparse binding for per-page virtual texture
   mapping.
   - **Rationale:** Loading only required mip levels minimizes GPU memory consumption while
     maintaining visual quality.
   - **Verification:** Render a scene with 1000 textures; verify only mips matching screen-space
     density are resident; move the camera closer and verify higher mips stream in within 500 ms;
     verify evicted mips free GPU memory.
5. **R-12.5.5** — The engine **SHALL** stream mesh LOD levels and meshlet groups on demand based on
   screen-space projected size and camera velocity, keeping coarse LODs permanently resident and
   loading fine LODs asynchronously, with dithered cross-fade transitions to prevent popping.
   - **Rationale:** On-demand mesh streaming enables open worlds with millions of unique meshes
     without requiring all geometry to be resident.
   - **Verification:** Place 10,000 meshes; verify coarse LODs at distance and fine LODs as camera
     approaches; capture frames during transitions and verify dithered cross-fade; benchmark GPU
     memory within the configured budget.

## Priority and Memory Management

6. **R-12.5.6** — The engine **SHALL** schedule all pending I/O requests through a priority queue
   ordered by screen-space size, camera distance, asset type weight, and frame deadline, coalescing
   adjacent requests and aging stale requests to prevent priority inversion.
   - **Rationale:** Priority-based scheduling ensures frame-critical assets load before background
     prefetch.
   - **Verification:** Submit 100 low-priority and 10 high-priority requests; verify all
     high-priority complete first; verify a stale request eventually promotes; verify coalescing
     reduces total I/O operations by at least 20%.
7. **R-12.5.7** — The engine **SHALL** monitor GPU and CPU memory usage against configurable
   thresholds and progressively evict lowest-priority assets, reduce streaming quality targets, and
   signal subsystems to release cached data when pressure exceeds the budget.
   - **Rationale:** Active memory pressure management prevents out-of-memory crashes during
     open-world traversal.
   - **Verification:** Configure a 512 MB budget; stream assets until the budget is reached; verify
     eviction and quality reduction without crashing; verify usage stays within 10% of budget over a
     120-second stress test.

## Archive and Compression

8. **R-12.5.8** — The engine **SHALL** pack processed assets into seekable archive files with a
   central directory providing O(1) lookup by asset ID, organized by streaming region and priority
   tier, supporting multiple simultaneous archive mounts for expansion packs and DLC.
   - **Rationale:** Seekable archives with spatial organization maximize sequential read throughput
     and eliminate per-file syscall overhead.
   - **Verification:** Pack 10,000 assets; verify O(1) lookup; mount two archives and verify both
     are accessible; benchmark sequential throughput at 90%+ of raw disk bandwidth.
9. **R-12.5.9** — The engine **SHALL** compress archive chunks using LZ4 for latency-sensitive
   assets (audio, UI) and Zstd for throughput-sensitive assets (textures, meshes), with chunk
   boundaries aligned to streaming granularity for independent per-asset decompression.
   - **Rationale:** Per-chunk compression with codec selection by asset type balances latency
     against compression ratio.
   - **Verification:** Compress and decompress assets of each type; verify integrity via content
     hashes; verify audio/UI use LZ4 and textures/meshes use Zstd; verify a single chunk
     decompresses independently.

## Download-on-Demand

10. **R-12.5.10** — The engine **SHALL** stream assets from a remote CDN on first access when not
    present locally, using a manifest mapping asset IDs to CDN URLs and BLAKE3 content hashes,
    verifying and decompressing downloaded chunks before writing them into the local archive.
    - **Rationale:** Download-on-demand enables players to begin playing immediately while content
      streams in background.
    - **Verification:** Remove an asset from local archive; access via VFS and verify it downloads
      from a mock CDN, passes hash verification, and is available locally for subsequent access;
      verify a corrupted download is rejected and retried.
