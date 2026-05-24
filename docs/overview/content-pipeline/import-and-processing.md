# Import and Processing

Bringing assets in and converting them to engine format.

## What it covers

- Native texture, audio, and mesh ingestion with format validation and hash verification.
- Offline texture compression to GPU-native formats (multiple platform-specific codecs).
- Automatic LOD generation via edge-collapse simplification.
- Meshlet building (≤64 vertices, 124 triangles) with GPU culling bounds.
- Lightmap UV atlas generation.
- Audio encoding (Opus, ADPCM, PCM) by asset type.
- Visual shader graph compilation to native code.
- Collision-shape generation (V-HACD, convex hulls, bounding primitives).
- Dependency DAG driving incremental rebuilds.
- Error reporting with file path, byte offset, and actionable fixes.

## Concepts

### Compression and Optimization

Textures are compressed offline to platform-specific formats: high-quality codec for desktop,
mobile-optimized codec for mobile and Apple, fallback codec for older platforms. Meshes are
automatically simplified into discrete LODs using edge-collapse with silhouette preservation.
Meshlets partition geometry into small clusters that GPUs can efficiently cull and shade.

### Dependency Graph

Assets reference other assets (material references texture, scene references mesh). The engine
builds a directed acyclic graph of dependencies. When a source asset changes, only affected
downstream assets are reprocessed, dramatically speeding up iteration.

### Import Settings

Per-asset import presets control compression, LOD targets, audio codec, and collision shape. Presets
are authored once per project and applied uniformly. Platform-specific overrides allow mobile to use
more aggressive compression than desktop.

## How it fits

- See [database-and-versioning](./database-and-versioning.md) for asset DB storage and lookup.
- See [streaming-and-hot-reload](./streaming-and-hot-reload.md) for asset runtime delivery.
- Integrates with [rendering](../rendering/pipeline.md), [audio](../audio/engine-and-mixing.md), and
  [physics](../physics/dynamics.md) for platform-specific asset formats.
