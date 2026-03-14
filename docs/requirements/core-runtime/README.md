# Core Runtime Requirements

Architecture, platform, performance, hardware, data constraint, resource budget, visual quality,
testing, and API/extensibility requirements for the Harmonius engine.

## Files

| File | Scope |
|------|-------|
| [architecture.md](architecture.md) | Safety, GPU-driven rendering, mesh shaders, render graph, cross-platform, layer separation |
| [platform-support.md](platform-support.md) | macOS/Metal, Windows/D3D12, Windows+SteamOS/Vulkan, platform-native IO |
| [rendering-pipeline.md](rendering-pipeline.md) | G-buffer layout, motion vectors, post-processing chain, TAA |
| [performance.md](performance.md) | Parallel encoding, barriers, aliasing, feature gates, streaming, triple-buffering |
| [hardware.md](hardware.md) | Bindless, mesh shaders, RT soft-gated, async compute, timeline fences, sparse textures |
| [data-constraints.md](data-constraints.md) | Reverse-Z, bone limits, alignment requirements, shader IR, asset formats |
| [resource-budgets.md](resource-budgets.md) | Meshlet limits, light capacity, shadow atlas, streaming pools, probe grids |
| [visual-quality.md](visual-quality.md) | Temporal stability, denoising, color precision, shadow bias, LOD transitions |
| [testing.md](testing.md) | Real GPU testing, TDD, snapshots, fuzz testing, CI, performance regression |
| [plugins.md](plugins.md) | User API, runtime quality config, error reporting, extensibility points |
