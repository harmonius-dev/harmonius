# Requirements

Hierarchical requirements for the hybrid render graph library. Each requirement has a unique identifier (e.g., `R-1.1.3` is the third requirement in section 1.1). Features reference requirements by ID (e.g., `R-1.1.2`).

## Sections

### 1 Architecture
- [1.1 Core Constraints](1-architecture/1.1-core-constraints.md) — safety, rendering paradigm, scope
- [1.2 Platform Support](1-architecture/1.2-platform-support.md) — target platforms and GPU APIs
- [1.3 Rendering Pipeline](1-architecture/1.3-rendering-pipeline.md) — HDR, G-buffer, motion vectors, post-processing, TAA

### 2 Nonfunctional
- [2.1 Performance](2-nonfunctional/2.1-performance.md) — rendering, memory, concurrency
- [2.2 Hardware](2-nonfunctional/2.2-hardware.md) — GPU capabilities and IO
- [2.3 Data Constraints](2-nonfunctional/2.3-data-constraints.md) — limits, conventions, formats
- [2.4 Resource Budgets](2-nonfunctional/2.4-resource-budgets.md) — meshlets, lights, shadows, streaming, probes, sprites, atlases
- [2.5 Visual Quality](2-nonfunctional/2.5-visual-quality.md) — temporal stability, denoising, color precision, LOD, atmosphere

### 3 Quality
- [3.1 Testing and CI](3-quality/3.1-testing-and-ci.md) — testing strategy, CI, instrumentation

### 4 API and Extensibility
- [4.1 User API](4-api-and-extensibility/4.1-user-api.md) — declarative scene, quality config, error reporting, resource handles
- [4.2 Extensibility](4-api-and-extensibility/4.2-extensibility.md) — shader nodes, custom passes, material extensions, animation evaluators
