# Content Pipeline Features

Asset import, processing, storage, hot reload, and streaming/IO systems for MMO-scale content.

## Files

| File | Scope |
|------|-------|
| [asset-import.md](asset-import.md) | glTF/FBX model import, texture import (PNG, JPEG, KTX2, DDS, HDR, EXR), audio import, animation/scene import, import presets |
| [asset-processing.md](asset-processing.md) | Texture compression (BC, ASTC, ETC), LOD generation, meshlet building, vertex optimization, lightmap UVs, audio encoding, shader cross-compilation, dependency graphs |
| [asset-database.md](asset-database.md) | Content-addressable storage, metadata, hash-based caching, incremental builds, search/tagging, thumbnails, AI categorization, LLM semantic search, smart collections, versioning |
| [hot-reload.md](hot-reload.md) | File watcher, asset/shader/script/UI hot reload, partial re-import, editor-runtime sync |
| [streaming-io.md](streaming-io.md) | Virtual file system, platform-native async I/O (IOCP/GCD/io_uring), GPU direct storage, texture/mesh streaming, priority queues, memory pressure, pak archives, compression, download-on-demand |
| [dcc-plugins.md](dcc-plugins.md) | Plugin SDK, live link, Houdini (3), Maya (2), Blender (2), Marvelous Designer (2), Substance (3), Photoshop (2), Illustrator (1), ZBrush (2), MotionBuilder (2), Git LFS lock/comments/dashboard (3), material mapping, batch CI — 26 features |
| [asset-versioning.md](asset-versioning.md) | Universal binary format, compressed bundles, structural diffing, three-way merge, auto conflict resolution, spreadsheet editor, universal inspector, Git LFS integration |
