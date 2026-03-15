# 12.5 — Streaming & I/O

## Virtual File System

### F-12.5.1 Virtual File System

An abstraction layer that presents a unified path namespace over multiple backing stores: loose
files on disk during development, pak/archive files in shipping builds, and remote HTTP stores for
download-on-demand patching. All engine subsystems access assets through the VFS, decoupling content
layout from physical storage.

- **Requirements:** R-12.5.1
- **Dependencies:** None
- **Platform notes:** None

## Async Asset Loading

### F-12.5.2 Platform-Native Async I/O

Asynchronous file reads using each platform's highest-throughput I/O API: I/O completion ports on
Windows, Grand Central Dispatch (dispatch_io) on macOS, and io_uring on Linux. All reads bypass the
CPU page cache via direct I/O for predictable latency. No standard library file I/O is used anywhere
in the engine. Builds on the core async I/O abstraction defined in F-1.8.1.

- **Requirements:** R-12.5.2
- **Dependencies:** F-12.5.1, F-1.8.1 (Platform I/O Backend Abstraction), F-1.8.3 (Async File I/O
  Operations)
- **Platform notes:** IOCP on Windows, GCD on macOS, io_uring on Linux. Each backend is a separate
  platform module behind a unified async I/O trait.

### F-12.5.3 GPU Direct Storage

File-to-GPU DMA that transfers compressed asset data from SSD directly into GPU memory, where a
compute shader decompresses it in place. Bypasses CPU involvement entirely for bulk asset transfers,
saturating NVMe bandwidth for open-world streaming.

- **Requirements:** R-12.5.3
- **Dependencies:** F-12.5.2
- **Platform notes:** Uses DirectStorage on Windows, Metal I/O on macOS. Linux falls back to
  io_uring with CPU-side staging.

## Streaming LOD

### F-12.5.4 Texture Streaming

Priority-based mip-level streaming that loads only the mip levels needed at the current screen-space
texel density. A residency manager tracks which mips are resident, schedules async loads for
requested mips, and evicts low-priority mips under memory pressure. Virtual textures use sparse
binding to map individual pages on demand.

- **Requirements:** R-12.5.4
- **Dependencies:** F-12.5.2, F-12.5.6
- **Platform notes:** Sparse texture binding required (Vulkan sparse resources, Metal sparse
  textures). Mobile uses tighter residency budgets (256 MB vs 1+ GB on desktop) and streams fewer
  mip levels concurrently due to slower storage throughput.

### F-12.5.5 Mesh Streaming

Stream mesh LOD levels and meshlet groups on demand based on screen-space projected size and camera
velocity. Coarse LODs are resident at all times; fine LODs stream in asynchronously as the camera
approaches. LOD transitions use dithered cross-fade to avoid popping across an open world with
millions of unique meshes.

- **Requirements:** R-12.5.5
- **Dependencies:** F-12.5.2, F-12.5.6
- **Platform notes:** Mobile keeps fewer fine LODs resident and uses more aggressive coarse LOD
  thresholds due to smaller memory budgets and slower I/O bandwidth.

## Priority and Memory Management

### F-12.5.6 Streaming Priority Queues

A priority queue schedules all pending I/O requests by urgency: screen-space size, distance from
camera, asset type weight, and frame deadline. The scheduler reorders and coalesces requests to
maximize throughput and minimize seek latency. Priority inversion is prevented by aging stale
requests.

- **Requirements:** R-12.5.6
- **Dependencies:** F-12.5.2
- **Platform notes:** None

### F-12.5.7 Memory Pressure Response

A budget monitor tracks GPU and CPU memory usage against configurable thresholds. When memory
pressure rises, the system progressively evicts lowest-priority streamed assets (distant mips,
off-screen LODs), reduces streaming quality targets, and signals subsystems to release cached data.
Prevents out-of-memory crashes during open-world traversal.

- **Requirements:** R-12.5.7
- **Dependencies:** F-12.5.4, F-12.5.5, F-12.5.6
- **Platform notes:** Mobile devices trigger memory pressure response at lower thresholds. iOS
  low-memory warnings (didReceiveMemoryWarning) and Android trim callbacks drive aggressive
  eviction.

## Archive and Compression

### F-12.5.8 Pak / Archive Files

Pack processed assets into seekable archive files with a central directory for O(1) lookup by asset
ID. Archives are organized by streaming region and priority tier so that sequential reads align with
spatial locality in the open world. Supports mounting multiple archives simultaneously for expansion
packs and DLC.

- **Requirements:** R-12.5.8
- **Dependencies:** F-12.5.1
- **Platform notes:** None

### F-12.5.9 Compression (LZ4, Zstd)

Per-chunk compression within archive files using LZ4 for latency-sensitive assets (audio, UI) and
Zstd for throughput-sensitive assets (textures, meshes). Chunk boundaries align with streaming
granularity so that individual assets can be decompressed independently without reading the entire
archive.

- **Requirements:** R-12.5.9
- **Dependencies:** F-12.5.8
- **Platform notes:** GPU decompression via F-12.5.3 supersedes CPU decompression where supported.

## Download-on-Demand

### F-12.5.10 Download-on-Demand Patching

Stream assets from a remote CDN on first access when they are not present in local archives. A
manifest file maps asset IDs to CDN URLs and content hashes. Downloaded chunks are verified,
decompressed, and written into the local archive for future access. Enables MMO-scale distribution
where players download only the content they need rather than the entire game upfront.

- **Requirements:** R-12.5.10
- **Dependencies:** F-12.5.1, F-12.5.8, F-12.5.9
- **Platform notes:** Mobile networks are slower and metered. Downloads use smaller chunk sizes,
  respect iOS/Android background download APIs, and pause on cellular by default.
