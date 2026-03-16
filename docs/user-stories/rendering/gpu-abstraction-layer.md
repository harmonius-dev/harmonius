# User Stories -- 2.1 GPU Abstraction Layer

## US-2.1.1.1 Write Rendering Code Once for All GPU Backends

**As an** engine developer (P-26), **I want** a unified GPU backend trait with associated types for
device, command buffer, pipeline state, and resource handles using static dispatch via generics,
**so that** I can write rendering code once without vtable overhead or backend-specific branching.

## US-2.1.1.2 Verify Backend Trait Conformance Across Metal, D3D12, and Vulkan

**As an** engine tester (P-27), **I want** to run the full rendering test suite on each backend and
diff the output images, **so that** I can confirm all three backends produce identical results from
the same trait-based rendering code.

## US-2.1.2.1 Record and Submit Command Buffers With Type-Safe Binding

**As an** engine developer (P-26), **I want** a trait-based command buffer abstraction supporting
graphics, compute, and copy operations with type-safe resource binding, **so that** I can encode
rendering work independently and submit ordered batches without unsafe casts.

## US-2.1.2.2 Validate Command Buffer Mapping to Platform APIs

**As an** engine tester (P-27), **I want** to trace command buffer submission and verify that Metal
uses MTLCommandBuffer, D3D12 uses ID3D12GraphicsCommandList, and Vulkan uses VkCommandBuffer,
**so that** the abstraction maps correctly to native APIs.

## US-2.1.3.1 Create Pipeline States With Zero-Cost Encoding

**As an** engine developer (P-26), **I want** pipeline state objects pre-validated at creation time
covering shaders, vertex layout, blend state, and depth-stencil configuration, **so that** command
buffer encoding has zero runtime validation cost.

## US-2.1.3.2 Verify PSO Cache Warming on Mobile at Load Time

**As an** engine tester (P-27), **I want** to confirm that pipeline state caches are warmed at load
time on tile-based mobile GPUs to avoid hitching during gameplay, **so that** PSO creation does not
cause frame drops on mobile.

## US-2.1.4.1 Implement Metal Backend via Swift-to-C-to-Bindgen

**As an** engine developer (P-26), **I want** the Metal GPU backend exposed as a Swift library with
@_cdecl C-compatible functions consumed by Rust through bindgen, **so that** Metal functionality is
available without Objective-C or C++ in the FFI boundary.

## US-2.1.4.2 Verify Metal Backend Builds on macOS and iOS Only

**As an** engine tester (P-27), **I want** to confirm that the Metal backend compiles only on macOS
and iOS targets and is excluded from Windows and Linux builds, **so that** platform gating is
correct.

## US-2.1.5.1 Implement D3D12 Backend via COM-to-Bindgen

**As an** engine developer (P-26), **I want** the D3D12 backend generated from C-compatible COM
headers via bindgen with safe Rust wrappers managing COM reference counting, **so that** D3D12 is
accessed without C++ or windows-rs dependencies.

## US-2.1.5.2 Verify D3D12 Backend Builds on Windows Only

**As an** engine tester (P-27), **I want** to confirm that the D3D12 backend compiles only on
Windows targets and is excluded from macOS and Linux builds, **so that** platform gating is correct.

## US-2.1.6.1 Implement Vulkan Backend With Validation Layer Integration

**As an** engine developer (P-26), **I want** a Vulkan backend generated from vulkan.h via bindgen
with RAII lifetime management and validation layers enabled in debug builds, **so that** runtime
Vulkan errors are caught during development.

## US-2.1.6.2 Verify Vulkan Loader Discovery at Runtime

**As an** engine tester (P-27), **I want** to confirm that the Vulkan loader is discovered at
runtime on Windows and Linux, and that MoltenVK is never used on macOS, **so that** Vulkan
initialization follows the specified platform strategy.

## US-2.1.7.1 Reduce GPU Memory Allocations With Sub-Allocation

**As an** engine developer (P-26), **I want** a GPU heap sub-allocator that carves typed buffer and
texture regions from pre-allocated memory blocks, **so that** thousands of per-draw constant uploads
use offset-based binding from a handful of OS allocations.

## US-2.1.7.2 Validate Sub-Allocator Alignment Per Backend

**As an** engine tester (P-27), **I want** to verify that sub-allocations respect 256-byte alignment
on D3D12, variable alignment on Vulkan, and page alignment on Metal, **so that** GPU memory access
is correctly aligned on each backend.

## US-2.1.8.1 Filter Redundant GPU State Transitions

**As an** engine developer (P-26), **I want** CPU-side shadow state tracking to filter redundant
pipeline, binding, and render target transitions before encoding, **so that** driver overhead is
minimized during high-frequency draw submission.

## US-2.1.8.2 Profile State Tracking Benefit on Mobile vs Desktop

**As an** engine tester (P-27), **I want** to measure draw submission cost with and without state
tracking on mobile (where per-change overhead is higher) and desktop, **so that** I can confirm the
state tracker provides measurable improvement on all platforms.

## US-2.1.9.1 Batch and Reorder Resource Barriers Automatically

**As an** engine developer (P-26), **I want** automatic barrier batching, merging, and split barrier
insertion within command buffers, **so that** consecutive transitions are coalesced and GPU pipeline
stalls are reduced.

## US-2.1.9.2 Verify Barrier Behavior as No-Op on Metal

**As an** engine tester (P-27), **I want** to confirm that barrier optimization is a no-op on Metal
where the driver handles hazard tracking, while D3D12 and Vulkan emit explicit barriers, **so that**
per-backend barrier behavior is correct.

## US-2.1.10.1 Enable GPU-Driven Dispatch Without CPU Round-Trips

**As an** engine developer (P-26), **I want** work graph support where GPU nodes dispatch work to
subsequent nodes without CPU involvement, **so that** GPU-driven rendering pipelines avoid
per-dispatch CPU latency.

## US-2.1.10.2 Validate Work Graph Emulation on Vulkan and Metal

**As an** engine tester (P-27), **I want** to verify that work graphs use native D3D12 API on
Windows and compute-based emulation via indirect dispatch chains on Vulkan and Metal, **so that**
the feature works consistently across all backends.

## US-2.1.11.1 Provide Consistent API When Backend Lacks Native Support

**As an** engine developer (P-26), **I want** a cross-backend feature emulation layer that selects
emulated paths at device creation time based on capability queries, **so that** mesh shaders, work
graphs, and enhanced barriers work on all backends without runtime branching.

## US-2.1.11.2 Test Emulated Mesh Shaders on Older Vulkan Drivers

**As an** engine tester (P-27), **I want** to run on Vulkan drivers that lack mesh shader support
and verify that the emulation path produces correct output matching native mesh shader results,
**so that** emulated features are visually identical to native.

## US-2.1.12.1 Profile Per-Pass GPU Time With Timestamp Queries

**As a** technical artist (P-13), **I want** per-pass GPU timestamps and pipeline statistics visible
in the editor's GPU profiler, **so that** I can identify which rendering passes are most expensive
and optimize art content accordingly.

## US-2.1.12.2 Verify GPU Profiling Queries Across All Backends

**As an** engine tester (P-27), **I want** to confirm that timestamp queries use
MTLCounterSampleBuffer on Metal, ID3D12QueryHeap on D3D12, and vkCmdWriteTimestamp on Vulkan,
**so that** GPU profiling data is accurate on each platform.

## US-2.1.12.3 Read Back Profiling Data One Frame Later to Avoid Stalls

**As an** engine developer (P-26), **I want** GPU timestamp query results read back one frame after
submission rather than the same frame, **so that** profiling does not introduce GPU stalls or
pipeline bubbles.
