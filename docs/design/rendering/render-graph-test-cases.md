# Render Graph Test Cases

Companion test cases for [render-graph.md](render-graph.md).

## Unit Tests

### TC-RG.13.4.1 Empty Graph Error

| # | Requirement |
|---|-------------|
| 1 | RG-13.4     |

1. **#1** — Empty graph (zero passes)
   - **Expected:** `EmptyGraph` error

### TC-RG.5.7.1 Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | RG-5.7      |

1. **#1** — Passes A -> B -> C -> A
   - **Expected:** `CycleDetected` error with handles {A, B, C}

### TC-RG.3.5.1 Single Writer Violation

| # | Requirement |
|---|-------------|
| 1 | RG-3.5      |

1. **#1** — Two passes writing same resource in overlapping window
   - **Expected:** `SingleWriterViolation` error

### TC-RG.13.4.2 Undeclared Resource

| # | Requirement |
|---|-------------|
| 1 | RG-13.4     |

1. **#1** — Pass reads resource not declared in any pass
   - **Expected:** `UndeclaredResource` error

### TC-RG.5.6.1 Topological Sort Stability

| # | Requirement |
|---|-------------|
| 1 | RG-5.6      |

1. **#1** — Same graph compiled twice
   - **Expected:** Identical pass ordering both times

### TC-RG.5.1.1 Topological Sort Correctness

| # | Requirement |
|---|-------------|
| 1 | RG-5.1      |

1. **#1** — Graph with known dependency chain
   - **Expected:** Every pass executes after all its dependencies

### TC-RG.13.2.1 Dead Pass Elimination

| # | Requirement |
|---|-------------|
| 1 | RG-13.2     |

1. **#1** — Pass whose outputs are unused
   - **Expected:** Pass eliminated from compiled plan

### TC-RG.13.3.1 Transitive Dead Pass

| # | Requirement |
|---|-------------|
| 1 | RG-13.3     |

1. **#1** — Chain of passes leading to unused output
   - **Expected:** Entire orphaned chain eliminated

### TC-RG.6.2.1 Capability Gate Soft

| # | Requirement |
|---|-------------|
| 1 | RG-6.2      |

1. **#1** — Soft-gated pass, capability absent
   - **Expected:** Pass pruned, no error

### TC-RG.6.2.2 Capability Gate Hard

| # | Requirement |
|---|-------------|
| 1 | RG-6.2      |

1. **#1** — Hard-gated pass, capability absent
   - **Expected:** `CapabilityNotMet` error

### TC-RG.6.3.1 Fallback Chain

| # | Requirement |
|---|-------------|
| 1 | RG-6.3      |

1. **#1** — Fallback chain with 3 variants, first two incapable
   - **Expected:** Third variant selected

### TC-RG.13.7.1 Variant Selection

| # | Requirement |
|---|-------------|
| 1 | RG-13.7     |
| 2 | RG-13.7     |
| 3 | RG-13.7     |

1. **#1** — Exactly one variant active per slot
   - **Expected:** No error
2. **#2** — Zero variants active for a slot
   - **Expected:** Error produced
3. **#3** — Two variants active for same slot
   - **Expected:** Error produced

### TC-RG.1.6.1 Conditional Enable

| # | Requirement |
|---|-------------|
| 1 | RG-1.6      |

1. **#1** — Pass disabled at runtime
   - **Expected:** Excluded from execution without recompilation

### TC-RG.1.6.2 Conditional Cascade

| # | Requirement |
|---|-------------|
| 1 | RG-1.6      |

1. **#1** — Pass disabled, has exclusive dependents
   - **Expected:** Exclusive dependents also disabled

### TC-RG.3.1.1 Barrier RAW

| # | Requirement |
|---|-------------|
| 1 | RG-3.1      |

1. **#1** — Read-after-write on resource
   - **Expected:** Exactly one barrier emitted

### TC-RG.3.2.1 Barrier WAW

| # | Requirement |
|---|-------------|
| 1 | RG-3.2      |

1. **#1** — Write-after-write on resource
   - **Expected:** Execution barrier emitted

### TC-RG.3.6.1 Barrier Merge

| # | Requirement |
|---|-------------|
| 1 | RG-3.6      |

1. **#1** — Compatible barriers at same point
   - **Expected:** Merged into single barrier

### TC-RG.3.6.2 Split Barrier

| # | Requirement |
|---|-------------|
| 1 | RG-3.6      |

1. **#1** — Split barrier scenario
   - **Expected:** Begin/end placed at optimal points

### TC-RG.3.4.1 Cross Queue Barrier

| # | Requirement |
|---|-------------|
| 1 | RG-3.4      |

1. **#1** — Resource transferred across queues
   - **Expected:** Release + acquire barrier pair emitted

### TC-RG.8.2.1 Aliasing Non-Overlapping

| # | Requirement |
|---|-------------|
| 1 | RG-8.2      |

1. **#1** — Two non-overlapping transient resources
   - **Expected:** Same heap offset assigned

### TC-RG.8.2.2 Aliasing Overlapping

| # | Requirement |
|---|-------------|
| 1 | RG-8.2      |

1. **#1** — Two overlapping transient resources
   - **Expected:** Distinct heap offsets assigned

### TC-RG.8.6.1 Aliasing Efficiency

| # | Requirement |
|---|-------------|
| 1 | RG-8.6      |

1. **#1** — Multiple transient resources with aliasing
   - **Expected:** Aliased heap < sum of all resource sizes

### TC-RG.4.1.1 Queue Affinity Graphics

| # | Requirement |
|---|-------------|
| 1 | RG-4.1      |

1. **#1** — Pass with graphics affinity
   - **Expected:** Assigned to graphics queue

### TC-RG.4.2.1 Queue Affinity Compute

| # | Requirement |
|---|-------------|
| 1 | RG-4.2      |

1. **#1** — Async-compute pass
   - **Expected:** Assigned to dedicated compute queue

### TC-RG.4.5.1 Queue Fallback

| # | Requirement |
|---|-------------|
| 1 | RG-4.5      |

1. **#1** — Compute pass, no dedicated compute queue
   - **Expected:** Falls back to graphics queue

### TC-RG.10.4.1 Encoding Groups

| # | Requirement |
|---|-------------|
| 1 | RG-10.4     |
| 2 | RG-10.4     |

1. **#1** — Independent passes
   - **Expected:** Same encoding group
2. **#2** — Dependent passes
   - **Expected:** Separate encoding groups

### TC-RG.9.5.1 Sub Graph Instances

| # | Requirement |
|---|-------------|
| 1 | RG-9.5      |

1. **#1** — 4-instance sub-graph
   - **Expected:** 4 exclusive resource sets

### TC-RG.9.3.1 Sub Graph Shared

| # | Requirement |
|---|-------------|
| 1 | RG-9.3      |

1. **#1** — Shared resource across instances
   - **Expected:** Single allocation

### TC-RG.9.4.1 Sub Graph Array Layer

| # | Requirement |
|---|-------------|
| 1 | RG-9.4      |

1. **#1** — 4-instance sub-graph
   - **Expected:** Each instance writes correct array layer (0-3)

### TC-RG.13.8.1 Instance Count Mismatch

| # | Requirement |
|---|-------------|
| 1 | RG-13.8     |

1. **#1** — Instance count > array layers
   - **Expected:** Error produced

### TC-RG.2.4.1 History Resource

| # | Requirement |
|---|-------------|
| 1 | RG-2.4      |

1. **#1** — History resource with current/previous handles
   - **Expected:** Both resolve to correct textures

### TC-RG.2.5.1 Resolution Scaled

| # | Requirement |
|---|-------------|
| 1 | RG-2.5      |

1. **#1** — resolution_scale=0.5, base=1920x1080
   - **Expected:** Resource dimensions = 960x540

### TC-RG.7.2.1 Budget Cull Lowest

| # | Requirement |
|---|-------------|
| 1 | RG-7.2      |

1. **#1** — Over-budget scenario
   - **Expected:** Lowest-priority pass culled first

### TC-RG.7.3.1 Budget Cascade

| # | Requirement |
|---|-------------|
| 1 | RG-7.3      |

1. **#1** — Culled pass with exclusive consumers
   - **Expected:** Consumers also culled

### TC-RG.7.2.2 Budget Never Cull Required

| # | Requirement |
|---|-------------|
| 1 | RG-7.2      |

1. **#1** — Required-priority pass, over budget
   - **Expected:** Pass not culled

### TC-RG.1.9.1 Render Area Constraint

| # | Requirement |
|---|-------------|
| 1 | RG-1.9      |

1. **#1** — Render area set on pass
   - **Expected:** Propagates to aliasing write coverage

### TC-RG.1.3.1 Pass Chain Order

| # | Requirement |
|---|-------------|
| 1 | RG-1.3      |

1. **#1** — Chain with 3 steps
   - **Expected:** Steps execute in declaration order

### TC-RG.1.7.1 Host Callback No GPU

| # | Requirement |
|---|-------------|
| 1 | RG-1.7      |

1. **#1** — Host callback pass
   - **Expected:** Zero GPU commands submitted

### TC-RG.12.1.1 Diagnostics Pass Timing

| # | Requirement |
|---|-------------|
| 1 | RG-12.1     |

1. **#1** — Diagnostics enabled
   - **Expected:** Timestamp query pairs emitted per pass

### TC-RG.12.7.1 Diagnostics Stripped

| # | Requirement |
|---|-------------|
| 1 | RG-12.7     |

1. **#1** — Diagnostics disabled
   - **Expected:** No timestamp queries emitted

## Integration Tests

### TC-RG.13.1.I1 Full Frame Graph

| # | Requirement |
|---|-------------|
| 1 | RG-13.1     |

1. **#1** — Realistic frame graph (shadow, GBuffer, lighting, post, present)
   - **Expected:** Correct barriers, queue assignment, aliasing

### TC-RG.9.1.I1 Multi-View Shadow Cascades

| # | Requirement |
|---|-------------|
| 1 | RG-9.1      |

1. **#1** — 4 shadow cascades, shared scene data
   - **Expected:** 4 instances, 1 shared allocation, 4 layer writes

### TC-RG.9.1.I2 VR Stereo Graph

| # | Requirement |
|---|-------------|
| 1 | RG-9.1      |

1. **#1** — VR stereo, 2 eye views sharing culling
   - **Expected:** Shared passes execute once

### TC-RG.10.1.I1 Parallel Encoding Correctness

| # | Requirement |
|---|-------------|
| 1 | RG-10.1     |

1. **#1** — 20-pass graph, 4 threads
   - **Expected:** Command buffer contents match sequential encoding

### TC-RG.11.1.I1 Streaming Fallback

| # | Requirement |
|---|-------------|
| 1 | RG-11.1     |
| 2 | RG-11.1     |

1. **#1** — Pass depends on streamed resource, before arrival
   - **Expected:** Placeholder bound
2. **#2** — Pass depends on streamed resource, after arrival
   - **Expected:** Real resource bound

### TC-RG.13.5.I1 Recompilation Trigger

| # | Requirement |
|---|-------------|
| 1 | RG-13.5     |
| 2 | RG-13.5     |

1. **#1** — Parameter change only
   - **Expected:** No recompilation
2. **#2** — New pass added
   - **Expected:** Recompilation triggered

### TC-RG.13.6.I1 Incremental Recompile

| # | Requirement |
|---|-------------|
| 1 | RG-13.6     |

1. **#1** — Residency change
   - **Expected:** Only affected barriers recompiled

### TC-RG.3.1.I1 D3D12 Barrier Mapping

| # | Requirement |
|---|-------------|
| 1 | RG-3.1      |

1. **#1** — D3D12 backend barriers
   - **Expected:** `ResourceBarrier` calls with correct state transitions

### TC-RG.3.1.I2 Vulkan Barrier Mapping

| # | Requirement |
|---|-------------|
| 1 | RG-3.1      |

1. **#1** — Vulkan backend barriers
   - **Expected:** `vkCmdPipelineBarrier2` with correct access masks

### TC-RG.3.1.I3 Metal No Intra-Queue Barriers

| # | Requirement |
|---|-------------|
| 1 | RG-3.1      |

1. **#1** — Metal backend, single queue
   - **Expected:** No barriers emitted (driver hazard tracking)

## Benchmarks

### TC-RG.13.1.B1 Graph Compilation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 passes | Compile time | < 1 ms | US-2.2.12.1 |

### TC-RG.1.1.B1 Per-Frame Execution Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full frame execution | Overhead | < 0.5 ms | US-2.2.1.2 |

### TC-RG.10.1.B1 Parallel Encoding Speedup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 threads | Speedup vs single-threaded | >= 4x | US-2.2.10.1 |

### TC-RG.8.1.B1 Aliasing Efficiency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Typical frame | Memory saved | >= 40% | US-2.2.4.1 |

### TC-RG.7.1.B1 Budget Culling Decision

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Budget evaluation | Decision time | < 50 us | US-2.2.8.1 |

### TC-RG.3.1.B1 Barrier Analysis

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 passes | Analysis time | < 200 us | US-2.2.5.1 |

### TC-RG.12.1.B1 Diagnostic Export

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full diagnostic export | Export time | < 1 ms | US-2.2.13.1 |
