# 6 Render Graph Library Requirements

Low-level requirements for the declarative render graph library. The render graph is a frame-invariant DAG of passes and resources, compiled once and re-executed each frame. It knows nothing about rendering features (bloom, shadows, hair, etc.) — only passes, resources, queues, barriers, scheduling, and gating.

These requirements were derived from the high-level renderer functional requirements (R-2.x.y) via systematic translation and deduplication. See [traceability-matrix.md](traceability-matrix.md) for the complete bidirectional mapping.

## Categories

| # | File | Reqs | Scope |
|---|------|------|-------|
| RG-1 | [6.1-pass-declaration.md](6.1-pass-declaration.md) | 14 | Pass types (raster, compute, RT dispatch, transfer, present, work-graph), typed I/O, ordered chains, variants, conditional passes, debug metadata, checkerboard resolve |
| RG-2 | [6.2-resource-management.md](6.2-resource-management.md) | 25 | Transient, persistent, imported, history resources; formats, usage flags, atlas sub-allocation, resolution-scaled dimensions, sparse textures, acceleration structures, multi-frame history, opacity micromaps |
| RG-3 | [6.3-barriers-and-sync.md](6.3-barriers-and-sync.md) | 6 | Automatic barrier insertion, layout transitions, barrier batching, cross-queue ownership transfers, single-writer enforcement |
| RG-4 | [6.4-queue-assignment.md](6.4-queue-assignment.md) | 6 | Per-pass queue affinity, async compute queue, transfer queue, cross-queue dependencies, queue fallback |
| RG-5 | [6.5-scheduling-and-ordering.md](6.5-scheduling-and-ordering.md) | 7 | Topological sort, sub-graph instantiation, explicit ordering, multi-frame dependencies, priority scheduling, cycle detection |
| RG-6 | [6.6-capability-gating.md](6.6-capability-gating.md) | 7 | Static hardware gates, hard/soft gates, fallback chains, capability descriptors, composite budget gates, path-conditioned variants |
| RG-7 | [6.7-budget-culling.md](6.7-budget-culling.md) | 6 | GPU timing gates, cost/priority annotations, cascading dead-pass elimination, resolution scaling, pool utilization gates |
| RG-8 | [6.8-resource-aliasing.md](6.8-resource-aliasing.md) | 6 | Lifetime intervals, aliased heap allocation, pool-based aliasing, staging ring aliasing, heap selection, memory diagnostics |
| RG-9 | [6.9-multi-view-execution.md](6.9-multi-view-execution.md) | 5 | Parameterized sub-graph templates, per-instance resources, shared resources, array-layer targeting, multi-instance compilation |
| RG-10 | [6.10-parallel-encoding.md](6.10-parallel-encoding.md) | 7 | Independent command buffers, thread-safe pools, parallel encoding, encoding dependency graph, ring buffers, timeline fences |
| RG-11 | [6.11-streaming-integration.md](6.11-streaming-integration.md) | 7 | Transfer queue passes, completion fences, residency tracking, fault-driven loading, eviction callbacks, priority scheduling |
| RG-12 | [6.12-diagnostics.md](6.12-diagnostics.md) | 7 | GPU timestamp queries, pipeline statistics, transfer throughput, queue depth, GPU readback, debug overlays, zero-overhead opt-out |
| RG-13 | [6.13-graph-compilation.md](6.13-graph-compilation.md) | 8 | DAG compilation, dead-pass elimination, transitive elimination, compile-time validation, recompilation triggers, incremental recompilation |
| RG-14 | [6.14-per-frame-execution.md](6.14-per-frame-execution.md) | 8 | Topology-data separation, per-frame binding, dynamic resolution, pass activation flags, frame index, transfer injection, residency map binding |

**Total: 119 render graph requirements** across 14 categories.

## Traceability

- [traceability-matrix.md](traceability-matrix.md) — Bidirectional mapping between high-level requirements (R-2.x.y) and the render graph requirements (RG-x.y)

## Requirement Format

Each requirement follows this structure:

- **ID:** `RG-{category}.{seq}` (e.g., RG-1.1, RG-2.23)
- **Title:** Descriptive name
- **SHALL statement:** Normative specification of the required behavior
- **Derived from:** List of R-2.x.y IDs that motivated this requirement
- **Rationale:** Why the requirement exists
