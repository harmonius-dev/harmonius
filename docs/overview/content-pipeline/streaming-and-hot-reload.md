# Streaming and Hot Reload

Loading assets on demand and updating without restart.

## What it covers

- Virtual File System unifying loose files, pak archives, and HTTP stores.
- Async asset reads bypassing the page cache with GPU direct-storage path.
- Mip-level texture streaming driven by screen-space texel density.
- Mesh LOD and meshlet streaming by projected size and camera velocity.
- Priority queue ordering I/O by screen-space size, distance, deadline.
- Memory-pressure response with eviction and quality reduction.
- Pak archives with O(1) lookup and per-chunk compression (LZ4/Zstd).
- Download-on-demand from CDN with manifest and integrity verification.
- File watcher delegating to platform notifications.
- Hot reload for assets, shaders, logic graphs, and UI within seconds.

## Concepts

### Virtual File System

Assets are referenced by logical path, which can come from loose files on disk, pak archives
(similar to ZIP), or HTTP URLs. Mounts enable expansions and DLC to overlay new content onto the
base game. The same logical path can be served from different backing stores; the VFS automatically
selects the appropriate one.

### Streaming and Budgets

Texture mips and mesh LODs stream in asynchronously. High-detail mips stream in as the camera
approaches; low-detail mips are immediately available. The streaming system respects a global memory
budget; when pressure rises, it evicts low-priority assets and lowers quality targets. Mobile uses
tighter budgets and respects OS low-memory notifications.

### Hot Reload

File changes trigger re-import and in-place patching. Textures swap via descriptor updates; meshes
swap via atomic pointer replacement. Shaders recompile only affected permutations and swap pipeline
state objects at the next frame boundary. Logic graphs and UI rebuild affected subtrees while
preserving state (scroll position, focus, animation progress).

## How it fits

- See [import-and-processing](./import-and-processing.md) for offline asset preparation.
- See [database-and-versioning](./database-and-versioning.md) for asset storage and retrieval.
- Integrates with [simulation-loop](../core-runtime/simulation-loop.md) for frame-boundary
  coordinated updates.
