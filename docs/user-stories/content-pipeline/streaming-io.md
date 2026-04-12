# User Stories -- 12.5 Streaming & I/O

## Virtual File System

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.5.1 | engine developer (P-26)    |
| US-12.5.2 | build engineer (P-16)      |

1. **US-12.5.1** — **As an** engine developer (P-26), **I want** a virtual file system presenting a
   unified path namespace over loose files, pak archives, and remote HTTP stores, **so that** all
   subsystems access assets through one abstraction.
2. **US-12.5.2** — **As a** build engineer (P-16), **I want** content layout decoupled from physical
   storage via the VFS, **so that** development loose-file layouts and shipping archives use the
   same asset paths.

## Async Asset Loading

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.5.3 | engine developer (P-26)    |
| US-12.5.4 | engine developer (P-26)    |

1. **US-12.5.3** — **As an** engine developer (P-26), **I want** all asset reads performed through
   Tokio async I/O with direct I/O bypassing the CPU page cache, **so that** asset loading never
   blocks worker threads.
2. **US-12.5.4** — **As an** engine developer (P-26), **I want** file-to-GPU DMA that transfers
   compressed data from SSD directly into GPU memory with compute shader decompression, **so that**
   bulk asset transfers bypass CPU involvement.

## Texture and Mesh Streaming

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.5.5 | environment artist (P-8)   |
| US-12.5.6 | environment artist (P-8)   |
| US-12.5.7 | technical artist (P-13)    |

1. **US-12.5.5** — **As an** environment artist (P-8), **I want** only the mip levels needed at the
   current screen-space density streamed in for textures, **so that** GPU memory is used efficiently
   without visible quality loss.
2. **US-12.5.6** — **As an** environment artist (P-8), **I want** mesh LOD levels and meshlet groups
   streamed on demand based on camera distance with dithered cross-fade transitions, **so that** LOD
   popping is eliminated in open worlds.
3. **US-12.5.7** — **As a** technical artist (P-13), **I want** coarse LODs permanently resident
   while fine LODs load asynchronously as the camera approaches, **so that** distant geometry is
   always visible.

## Priority and Memory Management

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.5.8 | engine developer (P-26)    |
| US-12.5.9 | engine developer (P-26)    |
| US-12.5.10 | technical artist (P-13)   |

1. **US-12.5.8** — **As an** engine developer (P-26), **I want** all pending I/O requests scheduled
   by a priority queue ordered by screen-space size, distance, and frame deadline, **so that**
   frame-critical assets load before background prefetch.
2. **US-12.5.9** — **As an** engine developer (P-26), **I want** adjacent I/O requests coalesced and
   stale requests aged to prevent priority inversion, **so that** throughput is maximized and no
   request starves indefinitely.
3. **US-12.5.10** — **As a** technical artist (P-13), **I want** GPU and CPU memory budgets enforced
   with progressive eviction of lowest-priority assets under memory pressure, **so that** open-world
   traversal does not cause out-of-memory crashes.

## Archive and Compression

| ID         | Persona                    |
|------------|----------------------------|
| US-12.5.11 | build engineer (P-16)      |
| US-12.5.12 | build engineer (P-16)      |
| US-12.5.13 | engine developer (P-26)    |

1. **US-12.5.11** — **As a** build engineer (P-16), **I want** processed assets packed into seekable
   archive files with O(1) lookup by asset ID organized by streaming region, **so that** sequential
   reads align with spatial locality.
2. **US-12.5.12** — **As a** build engineer (P-16), **I want** per-chunk compression with LZ4 for
   latency-sensitive assets and Zstd for throughput-sensitive assets, **so that** each asset type
   uses the optimal codec.
3. **US-12.5.13** — **As an** engine developer (P-26), **I want** chunk boundaries aligned to
   streaming granularity so individual assets decompress independently, **so that** single-asset
   access does not require reading the entire archive.

## Download-on-Demand

| ID         | Persona                    |
|------------|----------------------------|
| US-12.5.14 | build engineer (P-16)      |
| US-12.5.15 | game player (P-23)         |

1. **US-12.5.14** — **As a** build engineer (P-16), **I want** a manifest mapping asset IDs to CDN
   URLs and content hashes for download-on-demand patching, **so that** players download only the
   content they need.
2. **US-12.5.15** — **As a** game player (P-23), **I want** assets streamed from a CDN on first
   access, verified, and cached locally for future use, **so that** I can start playing without
   downloading the entire game upfront.

## GPU Direct Storage and Residency

| ID         | Persona                    |
|------------|----------------------------|
| US-12.5.16 | engine developer (P-26)    |
| US-12.5.17 | engine developer (P-26)    |
| US-12.5.18 | game developer (P-15)      |
| US-12.5.19 | game player (P-23)         |

1. **US-12.5.16** — **As an** engine developer (P-26), **I want** textures and meshes loaded
   directly from disk into GPU memory via DirectStorage, Metal I/O, or io_uring staging, **so that**
   streaming saturates NVMe bandwidth without consuming CPU memory bandwidth.
2. **US-12.5.17** — **As an** engine developer (P-26), **I want** GPU compute decompression of
   streamed assets in place, **so that** the CPU is free for gameplay and simulation during
   open-world traversal.
3. **US-12.5.18** — **As a** game developer (P-15), **I want** the residency manager to enforce
   per-asset-type memory budgets with LRU eviction and predictive camera-driven prefetch,
   **so that** open-world games stay within memory limits without manual unload calls.
4. **US-12.5.19** — **As a** game player (P-23), **I want** the engine to evict unused assets during
   scene transitions and on OS memory pressure, **so that** long play sessions do not crash from
   out-of-memory errors.
