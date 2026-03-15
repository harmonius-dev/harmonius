# Render Graph Library Requirements

> **Migration note:** This file was authored against an earlier requirement structure. Requirements
> RG-1.1 through RG-3.6 have full Derived-from, Rationale, and Verification sections. Requirements
> from RG-4 onward need these sections added and their Derived-from links updated to reference
> the current feature IDs (F-2.2.x) instead of the deleted R-2.x.y IDs. This migration is
> tracked as a future work item.

Low-level requirements for the declarative render graph library. The render graph is a
frame-invariant DAG of passes and resources, compiled once and re-executed each frame. It knows
nothing about rendering features (bloom, shadows, hair, etc.) -- only passes, resources, queues,
barriers, scheduling, and gating.

These requirements were derived from the high-level renderer functional requirements (R-2.x.y) via
systematic translation and deduplication. See
[traceability-matrix.md](../6-render-graph/traceability-matrix.md) for the complete bidirectional
mapping.

**Total: 119 render graph requirements** across 14 categories.

## RG-1: Pass Declaration

### RG-1.1 Typed Pass Descriptors

The render graph SHALL require each pass to declare a pass type drawn from the set {rasterization,
compute, ray-tracing-dispatch, acceleration-structure-build, transfer, msaa-resolve, present} along
with typed resource I/O bindings specifying access mode (read, write, read-write) and usage type
(color-attachment, depth-attachment, shader-read, storage-read, storage-write,
shading-rate-attachment, indirect-argument, acceleration-structure-read,
acceleration-structure-build-write) per resource. The compiler SHALL derive all barriers, layout
transitions, and scheduling constraints exclusively from these declarations.

**Derived from:** R-2.1.6, R-2.1.8, R-2.1.12, R-2.3.5, R-2.3.8, R-2.3.11, R-2.3.12, R-2.3.13,
R-2.4.12, R-2.5.2, R-2.5.3, R-2.5.10, R-2.6.1, R-2.6.3, R-2.6.5, R-2.6.9, R-2.6.11, R-2.7.1,
R-2.7.3, R-2.7.10, R-2.7.11, R-2.8.11, R-2.9.1, R-2.9.8, R-2.9.9, R-2.10.1, R-2.10.5, R-2.10.6,
R-2.11.6, R-2.12.3, R-2.12.5, R-2.13.1, R-2.13.2, R-2.13.3, R-2.13.4, R-2.13.5, R-2.13.6, R-2.13.7,
R-2.13.8, R-2.13.9, R-2.13.10, R-2.13.11, R-2.13.12, R-2.14.4, R-2.15.6, R-2.15.7, R-2.16.1,
R-2.17.1, R-2.17.6

### RG-1.2 User-Declared Custom Pass Registration

The render graph SHALL provide a pass declaration API through which external callers register pass
descriptors specifying typed resource I/O, a queue capability mask, and an opaque execute callback.
Custom passes SHALL be indistinguishable from built-in passes with respect to barrier insertion,
resource aliasing eligibility, scheduling, and dead-pass elimination.

**Derived from:** R-2.11.6, R-2.13.10

### RG-1.3 Ordered Pass Chain Declaration

The render graph SHALL support declaring an ordered sequence of passes as a named chain where passes
execute in declaration order and the chain as a whole participates in the wider DAG. Individual
passes within the chain SHALL be independently removable without invalidating the chain.
Intermediate resources between chain steps SHALL be automatically declared as transient.

**Derived from:** R-2.1.12, R-2.4.12, R-2.5.6, R-2.5.8, R-2.6.6, R-2.7.10, R-2.9.7, R-2.9.12,
R-2.10.6, R-2.10.8, R-2.13.1, R-2.13.2, R-2.16.5, R-2.17.6

### RG-1.4 Compile-Time Pass Variant Selection

The render graph SHALL support declaring two or more named pass variants under a single logical pass
slot, with exactly one variant selected at graph compile time based on a configuration enum.
Unselected variants and their exclusively-dependent resources SHALL be eliminated from the execution
plan.

**Derived from:** R-2.1.8, R-2.3.3, R-2.3.5, R-2.4.3, R-2.4.13, R-2.5.3, R-2.5.9, R-2.6.7, R-2.6.8,
R-2.14.1, R-2.14.3, R-2.14.4, R-2.14.5, R-2.15.3, R-2.15.8, R-2.17.10

### RG-1.5 Array-Slice-Targeted Pass Instances

The render graph SHALL support passes that declare a single 2D texture array as their color or depth
output and write to a specified layer index, so that N passes can populate N layers of one resource
without requiring N separate resource handles.

**Derived from:** R-2.1.7, R-2.2.5, R-2.3.1

### RG-1.6 Conditional Pass Declaration

The render graph SHALL support declaring passes with a runtime boolean enable flag that can be
toggled per frame without requiring full graph recompilation. Disabled passes SHALL be treated as
dead and subject to cascading elimination of their exclusive resources and exclusively-dependent
passes.

**Derived from:** R-2.1.7, R-2.1.12, R-2.2.11, R-2.3.7, R-2.4.4, R-2.5.8, R-2.6.3, R-2.6.5, R-2.7.6,
R-2.8.4, R-2.9.8, R-2.9.12, R-2.9.13, R-2.13.1, R-2.13.2, R-2.13.3, R-2.13.5, R-2.13.6, R-2.13.7,
R-2.13.8, R-2.13.9, R-2.13.11, R-2.13.12, R-2.14.1, R-2.14.3, R-2.15.7, R-2.16.5, R-2.17.1, R-2.17.6

### RG-1.7 Host Callback Pass

The render graph SHALL support a pass type whose execution body is a host-side callback invoked
after a specified predecessor pass's fence is signaled. Callback passes SHALL participate in
scheduling and dependency tracking but SHALL NOT submit GPU commands.

**Derived from:** R-2.12.3, R-2.12.4

### RG-1.8 Per-Pass Debug Metadata

Each pass declaration SHALL accept a stable human-readable name and optional descriptive tags. Names
SHALL appear in GPU capture labels, diagnostic outputs, and statistics records. Metadata SHALL be
registered at graph-build time at zero per-frame allocation cost.

**Derived from:** R-2.7.6, R-2.7.7, R-2.7.8, R-2.12.8

### RG-1.9 Per-Pass Render Area Constraint

The render graph SHALL support annotating a rasterization pass with a declared render area: an (x,
y, width, height) rectangle specifying the texel region the pass writes. The compiler SHALL use the
declared render area when computing write coverage for aliasing and barrier scoping.

**Derived from:** R-2.3.1, R-2.3.2

### RG-1.10 Per-Instance Conditional Enable on Sub-Graph Instances

The render graph SHALL support setting an independent per-frame boolean active flag on each
individual instance of a multi-instance sub-graph template. Inactive instances SHALL have their
exclusive resource allocations skipped and their command buffer encoding suppressed.

**Derived from:** R-2.3.7, R-2.3.11, R-2.15.7

### RG-1.11 Variable-Count Sub-Graph Instantiation

The render graph SHALL support a sub-graph template whose instantiation count for a given frame is
determined at per-frame build time, bounded by a compile-time maximum.

**Derived from:** R-2.3.10

### RG-1.12 Per-Instance Variant Parameter on Sub-Graph

The render graph SHALL support supplying a distinct compile-time variant selection value per
sub-graph instance for each variant-dispatch slot.

**Derived from:** R-2.14.5

### RG-1.13 GPU Work Graph Pass

The render graph SHALL support a pass type whose execution dispatches a GPU work graph where the GPU
self-schedules producer/consumer work items without host CPU involvement.

**Derived from:** R-2.7.13

### RG-1.14 Checkerboard Resolve Pass

The render graph SHALL support a pass type that consumes the current frame's half-resolution
checkerboard output and the previous frame's resolved output to reconstruct a full-resolution image.

**Derived from:** R-2.14.6

## RG-2: Resource Management

### RG-2.1 Transient Resource Declaration

The render graph SHALL allow passes to declare named transient texture and buffer resources with
explicit format, usage flags, and dimension expressions.

**Derived from:** R-2.1.6, R-2.1.8, R-2.1.10, R-2.7.10, R-2.7.11

### RG-2.2 Persistent Resource Declaration

The render graph SHALL support declaring named persistent resources that survive across frame
boundaries. Persistent resources SHALL NOT be eligible for intra-frame aliasing.

**Derived from:** R-2.1.7, R-2.6.6, R-2.7.5, R-2.7.18, R-2.8.5, R-2.8.9, R-2.9.1, R-2.9.9, R-2.12.2,
R-2.16.1, R-2.16.2, R-2.17.1, R-2.17.6

### RG-2.3 Imported External Resource Declaration

The render graph SHALL allow externally-managed resources to be imported as named handles with
explicit initial access state metadata.

**Derived from:** R-2.1.4, R-2.10.2, R-2.10.5, R-2.11.4, R-2.11.5

### RG-2.4 History Resource Declaration

The render graph SHALL support declaring a resource whose previous frame's contents are available as
a read-only input in the current frame.

**Derived from:** R-2.5.6, R-2.5.8, R-2.5.9, R-2.6.5, R-2.12.3, R-2.12.4, R-2.13.4, R-2.13.11,
R-2.14.1, R-2.14.2, R-2.17.6

### RG-2.5 Resolution-Scaled Resource Dimensions

The render graph SHALL allow resource dimensions to be declared as expressions relative to a named
resolution parameter.

**Derived from:** R-2.1.11, R-2.13.1, R-2.13.2, R-2.13.11, R-2.14.2

### RG-2.6 Texture Array Resource Declaration

The render graph SHALL support declaring a single resource as a 2D texture array with a configurable
layer count.

**Derived from:** R-2.1.7, R-2.2.5, R-2.2.10, R-2.3.1, R-2.5.8, R-2.8.10, R-2.15.7

### RG-2.7 Variant-Conditional Resource Sets

Resources exclusively produced by inactive compile-time variants SHALL not be allocated.

**Derived from:** R-2.1.8, R-2.3.3, R-2.3.5, R-2.14.5

### RG-2.8 Pool-Backed Fixed-Capacity Resource Pools

The render graph SHALL support allocating persistent resources from named fixed-capacity typed
pools.

**Derived from:** R-2.3.2, R-2.8.1, R-2.8.3, R-2.8.5, R-2.8.7, R-2.12.2

### RG-2.9 Sparse Texture Resource Declaration

The render graph SHALL support declaring resources with a sparse residency flag.

**Derived from:** R-2.3.10, R-2.8.1, R-2.8.3, R-2.8.5, R-2.8.7, R-2.12.3, R-2.12.4

### RG-2.10 through RG-2.25

See [original files](../6-render-graph/) for the complete resource management requirements (RG-2.10
staging buffers, RG-2.11 shared/exclusive annotation, RG-2.12 shading rate image, RG-2.13 indirect
argument buffer, RG-2.14 ring buffer, RG-2.15 bindless heap, RG-2.16 64-bit render target, RG-2.17
atlas sub-region, RG-2.18 acceleration structure, RG-2.19 conditional history invalidation, RG-2.20
multiple resolution parameters, RG-2.21 multi-sample render target, RG-2.22 mip-level targeting,
RG-2.23 fixed-capacity persistent texture, RG-2.24 multi-frame history chain, RG-2.25 opacity
micromap annotation).

## RG-3: Barriers and Synchronization

### RG-3.1 Automatic Read-After-Write Barriers

The render graph compiler SHALL automatically insert the minimal required memory and execution
barriers between any pass that writes a resource and any subsequent pass that reads it.

### RG-3.2 Automatic Write-After-Write Barriers

The render graph compiler SHALL detect and prevent write-after-write hazards with execution
barriers.

### RG-3.3 Automatic Layout Transition Tracking

The render graph compiler SHALL track and emit minimum layout transitions per usage site.

### RG-3.4 Cross-Queue Ownership Transfer Barriers

The render graph compiler SHALL automatically emit queue family ownership-transfer barrier pairs for
cross-queue resource transitions.

### RG-3.5 Single-Writer Invariant Enforcement

The render graph compiler SHALL statically verify single-writer per resource per overlapping
execution window.

### RG-3.6 Barrier Merging and Split Barriers

The render graph compiler SHALL merge compatible barriers and prefer split barriers where supported.

## RG-4: Queue Assignment

### RG-4.1 Per-Pass Queue Affinity Declaration

Each pass SHALL declare queue affinity from {graphics, async-compute, transfer, any}.

### RG-4.2 Dedicated Async Compute Queue

The render graph SHALL expose a named async-compute queue slot for GPU-side compute/graphics
overlap.

### RG-4.3 Dedicated Transfer Queue

The render graph SHALL expose transfer queue slots operating concurrently with other queues.

### RG-4.4 Cross-Queue Dependency Declaration

Explicit dependency edges between passes on different queues, translated to timeline fence
signal/wait operations.

### RG-4.5 Queue Capability Fallback

Passes on unavailable queue types SHALL automatically fall back to the graphics queue.

### RG-4.6 Queue-Specific Command Buffer Allocation

Per-queue-type pools of command buffers without cross-queue locking.

## RG-5: Scheduling and Ordering

### RG-5.1 Topological Execution Order

Execution order derived from resource read/write edges via topological sort.

### RG-5.2 Parameterized Sub-Graph Instantiation

Reusable sub-graph templates instantiable N times per frame with different parameters.

### RG-5.3 Explicit Ordering Constraints

Ordering edges between passes with no resource dependency.

### RG-5.4 Multi-Frame Pass Dependencies

Cross-frame dependency edges for multi-frame async compute work.

### RG-5.5 Priority-Ordered Transfer Scheduling

Integer priority values for transfer pass ordering within submission.

### RG-5.6 Deterministic Ordering

Deterministic topological ordering across frames for reproducibility.

### RG-5.7 Cycle Detection

Structured error for cycles detected during compilation.

## RG-6: Capability Gating

### RG-6.1 Static Capability Gate on Passes

Pass exclusion at compile time based on hardware capability descriptors.

### RG-6.2 Hard vs. Soft Capability Gates

Hard gates fail compilation; soft gates silently prune passes.

### RG-6.3 Fallback Chain Declaration

Ordered fallback chains with capability-based selection.

### RG-6.4 Capability Descriptor

Typed enumeration of queryable hardware capabilities.

### RG-6.5 Queue Capability Fallback Gate

Automatic queue fallback without topology changes.

### RG-6.6 Composite Capability-and-Budget Fallback Gate

Conjunctive capability AND budget gate conditions per fallback chain entry.

### RG-6.7 Path-Conditioned Variant Gate

Variant exclusion based on active pipeline-path variant.

## RG-7: Budget Culling

### RG-7.1 GPU Timing Feedback Gate

Dynamic gate based on GPU timestamp delta from prior frames.

### RG-7.2 Per-Pass Cost and Priority Annotations

Cost and priority annotations for intelligent culling under frame-time pressure.

### RG-7.3 Cascading Dead-Pass Elimination

Transitive culling of downstream passes when a producer is culled.

### RG-7.4 Resolution Scale Parameter

Per-frame scalar resolution-scale parameter applied to resolution-scaled resources.

### RG-7.5 Per-Pool Utilization Budget Gate

Pool utilization threshold gates for culling low-priority work.

### RG-7.6 Runtime Parameter Mutation Without Recompilation

Dynamic gate parameters, resolution scale, and enable flags updatable per frame.

## RG-8: Resource Aliasing

### RG-8.1 Lifetime Interval Computation

First-write and last-read interval computation for transient resources.

### RG-8.2 Aliased Heap Allocation

Non-overlapping transient resources sharing backing heap memory.

### RG-8.3 Pool-Based Aliasing Domain

Intra-pool aliasing for fixed-capacity resource pools.

### RG-8.4 Staging Buffer Ring Aliasing

Ring buffer aliasing with per-frame slot recycling.

### RG-8.5 Heap Type Selection

Automatic memory heap type selection based on usage flags.

### RG-8.6 Memory Usage Diagnostics

Per-frame peak aliased memory, total allocated, and aliasing efficiency metrics.

## RG-9: Multi-View Execution

### RG-9.1 Parameterized Sub-Graph Templates

Reusable templates instantiable N times with exclusive and shared resource bindings.

### RG-9.2 Per-Instance Exclusive Resource Binding

Independent allocations per sub-graph instance for exclusive resources.

### RG-9.3 Shared Read-Only Resources Across Instances

Single physical allocation for shared inputs with merged barriers.

### RG-9.4 Array-Layer Instance Targeting

Per-instance layer targeting of shared texture array outputs.

### RG-9.5 Multi-Instance Sub-Graph Compilation

Combined DAG compilation across all instances for global optimization.

## RG-10: Parallel Encoding

### RG-10.1 Independent Command Buffers Per Pass

Independent command buffer per pass or pass group with no shared writes.

### RG-10.2 Thread-Safe Command Buffer Pool

Per-queue-type lock-free pools for encoding thread command buffer acquisition.

### RG-10.3 Sub-Graph Instance Parallel Encoding

Concurrent encoding of independent sub-graph instances on separate threads.

### RG-10.4 Encoding Dependency Graph

Graph identifying parallelizable and serialized encoding passes.

### RG-10.5 Per-Frame Ring Buffer Allocation

Lock-free ring buffer allocation for per-frame transient data.

### RG-10.6 Timeline Fence Coordination

Timeline fence primitives for cross-queue completion coordination.

### RG-10.7 Submission Ordering Separate from Encoding Order

Topological submission order independent of parallel encoding order.

## RG-11: Streaming Integration

### RG-11.1 Transfer Queue Upload Pass

Transfer-queue passes copying from staging to device-local resources.

### RG-11.2 Completion Fence Per Transfer Pass

Per-pass completion fence for host polling and dynamic gate input.

### RG-11.3 Residency Tracking Resource

Bindable residency-map buffers for GPU-side residency queries.

### RG-11.4 Fault-Driven Transfer Insertion

Runtime transfer pass injection without full graph recompilation.

### RG-11.5 LRU Eviction Callback

Eviction callback invocation on pool capacity exhaustion.

### RG-11.6 Transfer Pass Priority Scheduling

Priority-ordered concurrent transfer pass submission.

### RG-11.7 Frame-Boundary Resource Hand-Off

Cross-frame resource transfer with timeline fence protection.

## RG-12: Diagnostics

### RG-12.1 Per-Pass GPU Timestamp Queries

Named timestamp query pairs (begin/end) per pass.

### RG-12.2 Per-Pass Pipeline Statistics Queries

Named pipeline statistics query objects per pass.

### RG-12.3 Transfer Throughput Statistics

Per-transfer-pass byte counts and latency metrics.

### RG-12.4 Queue Depth Counter

Per-queue in-flight pass counters readable without GPU sync.

### RG-12.5 GPU Readback Pass

Pass type for GPU-to-CPU buffer copy with fence-gated host read.

### RG-12.6 Conditional Debug Overlay Passes

Runtime-togglable debug overlays with zero-cost when disabled.

### RG-12.7 Zero-Overhead Diagnostic Opt-Out

Per-query and per-pass diagnostic opt-out at compile time.

## RG-13: Graph Compilation

### RG-13.1 DAG Compilation to Execution Plan

Compilation producing topologically sorted pass sequence, barrier schedule, aliasing assignment, and
command buffer partitioning.

### RG-13.2 Dead-Pass Elimination

Removal of passes whose outputs are not consumed.

### RG-13.3 Transitive Dead-Pass Elimination

Cascading elimination of orphaned producer passes.

### RG-13.4 Compile-Time Validation

Structured errors for cycles, type mismatches, undeclared resources, unsatisfiable queues, and
single-writer violations.

### RG-13.5 Recompilation Triggers

Explicit trigger set (config enum, capability set, topology) with per-frame changes excluded.

### RG-13.6 Incremental Recompilation on Residency Change

Partial recompilation recomputing only affected barriers and transitions.

### RG-13.7 Variant Selection Validation

Compile-time enforcement of exactly one variant per variant-dispatch slot.

### RG-13.8 Sub-Graph Instance Count Validation

Compile-time layer count vs. instantiation count verification.

## RG-14: Per-Frame Execution

### RG-14.1 Topology-Data Separation

Strict separation of compiled topology from per-frame data.

### RG-14.2 Per-Frame Buffer and Texture Binding

Per-frame data interface for supplying new handles and constants without recompilation.

### RG-14.3 Per-Frame Sub-Graph Parameter Update

Per-instance parameter updates before execution.

### RG-14.4 Dynamic Resolution Parameter Update

Per-frame scalar resolution-scale parameter.

### RG-14.5 Per-Frame Pass Activation Flags

Boolean active flags on conditionally-declared passes.

### RG-14.6 Frame Index Propagation

Monotonically incrementing frame index for temporal patterns.

### RG-14.7 Dynamic Transfer Pass Injection

Post-compilation transfer pass insertion at designated injection points.

### RG-14.8 Per-Frame Residency Map Binding

Per-frame residency map rebinding without recompilation.
