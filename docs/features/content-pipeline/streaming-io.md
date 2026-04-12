# 12.5 — Streaming & I/O

## Virtual File System

| ID       | Feature             |
|----------|---------------------|
| F-12.5.1 | Virtual File System |

1. **F-12.5.1** — An abstraction layer that presents a unified path namespace over multiple backing
   stores: loose files on disk during development, pak/archive files in shipping builds, and remote
   HTTP stores for download-on-demand patching. All engine subsystems access assets through the VFS,
   decoupling content layout from physical storage.

## Async Asset Loading

| ID       | Feature                   |
|----------|---------------------------|
| F-12.5.2 | Platform-Native Async I/O |
| F-12.5.3 | GPU Direct Storage        |

1. **F-12.5.2** — Asynchronous file reads using Tokio async I/O (IOCP on Windows, kqueue on macOS,
   epoll on Linux internally). All reads bypass the CPU page cache via direct I/O for predictable
   latency. No standard library file I/O is used anywhere in the engine. Builds on the core async
   I/O abstraction defined in F-1.8.1 behind a unified async I/O trait.
   - **Deps:** F-12.5.1, F-1.8.1 (Platform I/O Backend Abstraction), F-1.8.3 (Async File I/O
   - **Platform:** Tokio handles platform I/O internally.
2. **F-12.5.3** — File-to-GPU DMA that transfers compressed asset data from SSD directly into GPU
   memory, where a compute shader decompresses it in place. Bypasses CPU involvement entirely for
   bulk asset transfers, saturating NVMe bandwidth for open-world streaming. Tokio with CPU-side
   staging.
   - **Deps:** F-12.5.2
   - **Platform:** Uses DirectStorage on Windows, Metal I/O on macOS. Linux falls back to

## Streaming LOD

| ID       | Feature           |
|----------|-------------------|
| F-12.5.4 | Texture Streaming |
| F-12.5.5 | Mesh Streaming    |

1. **F-12.5.4** — Priority-based mip-level streaming that loads only the mip levels needed at the
   current screen-space texel density. A residency manager tracks which mips are resident, schedules
   async loads for requested mips, and evicts low-priority mips under memory pressure. Virtual
   textures use sparse binding to map individual pages on demand. textures). Mobile uses tighter
   residency budgets (256 MB vs 1+ GB on desktop) and streams fewer mip levels concurrently due to
   slower storage throughput.
   - **Deps:** F-12.5.2, F-12.5.6
   - **Platform:** Sparse texture binding required (Vulkan sparse resources, Metal sparse
2. **F-12.5.5** — Stream mesh LOD levels and meshlet groups on demand based on screen-space
   projected size and camera velocity. Coarse LODs are resident at all times; fine LODs stream in
   asynchronously as the camera approaches. LOD transitions use dithered cross-fade to avoid popping
   across an open world with millions of unique meshes. thresholds due to smaller memory budgets and
   slower I/O bandwidth.
   - **Deps:** F-12.5.2, F-12.5.6
   - **Platform:** Mobile keeps fewer fine LODs resident and uses more aggressive coarse LOD

## Priority and Memory Management

| ID       | Feature                   |
|----------|---------------------------|
| F-12.5.6 | Streaming Priority Queues |
| F-12.5.7 | Memory Pressure Response  |

1. **F-12.5.6** — A priority queue schedules all pending I/O requests by urgency: screen-space size,
   distance from camera, asset type weight, and frame deadline. The scheduler reorders and coalesces
   requests to maximize throughput and minimize seek latency. Priority inversion is prevented by
   aging stale requests.
   - **Deps:** F-12.5.2
2. **F-12.5.7** — A budget monitor tracks GPU and CPU memory usage against configurable thresholds.
   When memory pressure rises, the system progressively evicts lowest-priority streamed assets
   (distant mips, off-screen LODs), reduces streaming quality targets, and signals subsystems to
   release cached data. Prevents out-of-memory crashes during open-world traversal. low-memory
   warnings (didReceiveMemoryWarning) and Android trim callbacks drive aggressive eviction.
   - **Deps:** F-12.5.4, F-12.5.5, F-12.5.6
   - **Platform:** Mobile devices trigger memory pressure response at lower thresholds. iOS

## Archive and Compression

| ID       | Feature                 |
|----------|-------------------------|
| F-12.5.8 | Pak / Archive Files     |
| F-12.5.9 | Compression (LZ4, Zstd) |

1. **F-12.5.8** — Pack processed assets into seekable archive files with a central directory for
   O(1) lookup by asset ID. Archives are organized by streaming region and priority tier so that
   sequential reads align with spatial locality in the open world. Supports mounting multiple
   archives simultaneously for expansion packs and DLC.
   - **Deps:** F-12.5.1
2. **F-12.5.9** — Per-chunk compression within archive files using LZ4 for latency-sensitive assets
   (audio, UI) and Zstd for throughput-sensitive assets (textures, meshes). Chunk boundaries align
   with streaming granularity so that individual assets can be decompressed independently without
   reading the entire archive.
   - **Deps:** F-12.5.8
   - **Platform:** GPU decompression via F-12.5.3 supersedes CPU decompression where supported.

## Download-on-Demand

| ID        | Feature                     |
|-----------|-----------------------------|
| F-12.5.10 | Download-on-Demand Patching |

1. **F-12.5.10** — Stream assets from a remote CDN on first access when they are not present in
   local archives. A manifest file maps asset IDs to CDN URLs and content hashes. Downloaded chunks
   are verified, decompressed, and written into the local archive for future access. Enables
   MMO-scale distribution where players download only the content they need rather than the entire
   game upfront. respect iOS/Android background download APIs, and pause on cellular by default.
   - **Deps:** F-12.5.1, F-12.5.8, F-12.5.9
   - **Platform:** Mobile networks are slower and metered. Downloads use smaller chunk sizes,

## GPU Direct Storage and Residency

| ID        | Feature                         |
|-----------|---------------------------------|
| F-12.5.11 | GPU Direct Storage Asset Loading|
| F-12.5.12 | Resource Residency Manager      |

1. **F-12.5.11** — Disk-to-GPU asset loading that bypasses CPU memory entirely for textures and
   meshes. Uses DirectStorage on Windows, Metal I/O on Apple platforms, and io_uring staging buffers
   on Linux to transfer compressed payloads from NVMe straight into GPU-visible memory where
   decompression runs on a compute shader. Saturates NVMe bandwidth during open-world streaming
   while leaving CPU bandwidth free for gameplay and simulation.
   - **Deps:** F-12.5.2, F-12.5.9
   - **Platform:** Windows uses DirectStorage 1.2+. Apple platforms use Metal I/O via
     `MTLIOCommandQueue`. Linux uses io_uring with GPU-visible staging buffers. Mobile tiers fall
     back to CPU staging when GPU DMA is unavailable.
2. **F-12.5.12** — Central residency manager tracking per-asset-type memory budgets (textures,
   meshes, audio, animations) with LRU eviction, predictive prefetching driven by camera position
   and velocity, scene transition bulk unloads, and emergency eviction triggered by OS memory
   pressure signals. The manager exposes budget queries to the streaming scheduler and enforces hard
   ceilings to prevent out-of-memory crashes during traversal.
   - **Deps:** F-12.5.6, F-12.5.7
   - **Platform:** Mobile uses tighter per-type budgets (textures 256 MB, meshes 128 MB). iOS hooks
     `didReceiveMemoryWarning`; Android hooks `onTrimMemory` for emergency eviction. Desktop budgets
     scale with available VRAM and system RAM.
