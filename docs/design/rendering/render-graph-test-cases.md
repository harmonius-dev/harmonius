# Render Graph Test Cases

Companion test cases for [render-graph.md](render-graph.md).

## Unit Tests

### TC-RG.13.4.1 Empty Graph Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Empty graph (zero passes) | `EmptyGraph` error | RG-13.4 |

### TC-RG.5.7.1 Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Passes A -> B -> C -> A | `CycleDetected` error with handles {A, B, C} | RG-5.7 |

### TC-RG.3.5.1 Single Writer Violation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two passes writing same resource in overlapping window | `SingleWriterViolation` error | RG-3.5 |

### TC-RG.13.4.2 Undeclared Resource

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass reads resource not declared in any pass | `UndeclaredResource` error | RG-13.4 |

### TC-RG.5.6.1 Topological Sort Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same graph compiled twice | Identical pass ordering both times | RG-5.6 |

### TC-RG.5.1.1 Topological Sort Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with known dependency chain | Every pass executes after all its dependencies | RG-5.1 |

### TC-RG.13.2.1 Dead Pass Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass whose outputs are unused | Pass eliminated from compiled plan | RG-13.2 |

### TC-RG.13.3.1 Transitive Dead Pass

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Chain of passes leading to unused output | Entire orphaned chain eliminated | RG-13.3 |

### TC-RG.6.2.1 Capability Gate Soft

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Soft-gated pass, capability absent | Pass pruned, no error | RG-6.2 |

### TC-RG.6.2.2 Capability Gate Hard

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hard-gated pass, capability absent | `CapabilityNotMet` error | RG-6.2 |

### TC-RG.6.3.1 Fallback Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fallback chain with 3 variants, first two incapable | Third variant selected | RG-6.3 |

### TC-RG.13.7.1 Variant Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exactly one variant active per slot | No error | RG-13.7 |
| 2 | Zero variants active for a slot | Error produced | RG-13.7 |
| 3 | Two variants active for same slot | Error produced | RG-13.7 |

### TC-RG.1.6.1 Conditional Enable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass disabled at runtime | Excluded from execution without recompilation | RG-1.6 |

### TC-RG.1.6.2 Conditional Cascade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass disabled, has exclusive dependents | Exclusive dependents also disabled | RG-1.6 |

### TC-RG.3.1.1 Barrier RAW

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Read-after-write on resource | Exactly one barrier emitted | RG-3.1 |

### TC-RG.3.2.1 Barrier WAW

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write-after-write on resource | Execution barrier emitted | RG-3.2 |

### TC-RG.3.6.1 Barrier Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compatible barriers at same point | Merged into single barrier | RG-3.6 |

### TC-RG.3.6.2 Split Barrier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Split barrier scenario | Begin/end placed at optimal points | RG-3.6 |

### TC-RG.3.4.1 Cross Queue Barrier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Resource transferred across queues | Release + acquire barrier pair emitted | RG-3.4 |

### TC-RG.8.2.1 Aliasing Non-Overlapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two non-overlapping transient resources | Same heap offset assigned | RG-8.2 |

### TC-RG.8.2.2 Aliasing Overlapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two overlapping transient resources | Distinct heap offsets assigned | RG-8.2 |

### TC-RG.8.6.1 Aliasing Efficiency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Multiple transient resources with aliasing | Aliased heap < sum of all resource sizes | RG-8.6 |

### TC-RG.4.1.1 Queue Affinity Graphics

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass with graphics affinity | Assigned to graphics queue | RG-4.1 |

### TC-RG.4.2.1 Queue Affinity Compute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Async-compute pass | Assigned to dedicated compute queue | RG-4.2 |

### TC-RG.4.5.1 Queue Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compute pass, no dedicated compute queue | Falls back to graphics queue | RG-4.5 |

### TC-RG.10.4.1 Encoding Groups

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Independent passes | Same encoding group | RG-10.4 |
| 2 | Dependent passes | Separate encoding groups | RG-10.4 |

### TC-RG.9.5.1 Sub Graph Instances

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-instance sub-graph | 4 exclusive resource sets | RG-9.5 |

### TC-RG.9.3.1 Sub Graph Shared

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shared resource across instances | Single allocation | RG-9.3 |

### TC-RG.9.4.1 Sub Graph Array Layer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-instance sub-graph | Each instance writes correct array layer (0-3) | RG-9.4 |

### TC-RG.13.8.1 Instance Count Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instance count > array layers | Error produced | RG-13.8 |

### TC-RG.2.4.1 History Resource

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | History resource with current/previous handles | Both resolve to correct textures | RG-2.4 |

### TC-RG.2.5.1 Resolution Scaled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | resolution_scale=0.5, base=1920x1080 | Resource dimensions = 960x540 | RG-2.5 |

### TC-RG.7.2.1 Budget Cull Lowest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Over-budget scenario | Lowest-priority pass culled first | RG-7.2 |

### TC-RG.7.3.1 Budget Cascade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Culled pass with exclusive consumers | Consumers also culled | RG-7.3 |

### TC-RG.7.2.2 Budget Never Cull Required

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Required-priority pass, over budget | Pass not culled | RG-7.2 |

### TC-RG.1.9.1 Render Area Constraint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render area set on pass | Propagates to aliasing write coverage | RG-1.9 |

### TC-RG.1.3.1 Pass Chain Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Chain with 3 steps | Steps execute in declaration order | RG-1.3 |

### TC-RG.1.7.1 Host Callback No GPU

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Host callback pass | Zero GPU commands submitted | RG-1.7 |

### TC-RG.12.1.1 Diagnostics Pass Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diagnostics enabled | Timestamp query pairs emitted per pass | RG-12.1 |

### TC-RG.12.7.1 Diagnostics Stripped

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diagnostics disabled | No timestamp queries emitted | RG-12.7 |

## Integration Tests

### TC-RG.13.1.I1 Full Frame Graph

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Realistic frame graph (shadow, GBuffer, lighting, post, present) | Correct barriers, queue assignment, aliasing | RG-13.1 |

### TC-RG.9.1.I1 Multi-View Shadow Cascades

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 shadow cascades, shared scene data | 4 instances, 1 shared allocation, 4 layer writes | RG-9.1 |

### TC-RG.9.1.I2 VR Stereo Graph

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | VR stereo, 2 eye views sharing culling | Shared passes execute once | RG-9.1 |

### TC-RG.10.1.I1 Parallel Encoding Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-pass graph, 4 threads | Command buffer contents match sequential encoding | RG-10.1 |

### TC-RG.11.1.I1 Streaming Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass depends on streamed resource, before arrival | Placeholder bound | RG-11.1 |
| 2 | Pass depends on streamed resource, after arrival | Real resource bound | RG-11.1 |

### TC-RG.13.5.I1 Recompilation Trigger

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parameter change only | No recompilation | RG-13.5 |
| 2 | New pass added | Recompilation triggered | RG-13.5 |

### TC-RG.13.6.I1 Incremental Recompile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Residency change | Only affected barriers recompiled | RG-13.6 |

### TC-RG.3.1.I1 D3D12 Barrier Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | D3D12 backend barriers | `ResourceBarrier` calls with correct state transitions | RG-3.1 |

### TC-RG.3.1.I2 Vulkan Barrier Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vulkan backend barriers | `vkCmdPipelineBarrier2` with correct access masks | RG-3.1 |

### TC-RG.3.1.I3 Metal No Intra-Queue Barriers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Metal backend, single queue | No barriers emitted (driver hazard tracking) | RG-3.1 |

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
