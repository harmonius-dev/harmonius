# User Stories -- 2.2 Render Graph

## US-2.2.1.1 Declare Render Passes Without Manual Barrier Management

**As an** engine developer (P-26), **I want** to declare passes as structs implementing a pass trait
with typed resource reads and writes, **so that** the graph compiler discovers dependency topology
and I never manually insert barriers or order passes.

## US-2.2.1.2 Verify Pass Registration Has Zero GPU Cost

**As an** engine tester (P-27), **I want** to confirm that pass registration is a CPU-side operation
with no GPU overhead by measuring GPU timeline during graph construction, **so that** graph setup
does not affect frame time.

## US-2.2.2.1 Automatically Fall Back When GPU Lacks Required Capabilities

**As an** engine developer (P-26), **I want** each pass to declare its required GPU capabilities
(mesh shaders, RT, compute) so the compiler prunes unsupported passes and activates fallback
implementations, **so that** the render graph adapts to any hardware configuration automatically.

## US-2.2.2.2 Validate RT Pass Pruning on Non-RT Hardware

**As an** engine tester (P-27), **I want** to run on a GPU without RT support and verify that all
RT-dependent passes are pruned and replaced by rasterization fallbacks, **so that** the render graph
compiles correctly without RT hardware.

## US-2.2.3.1 Declare Virtual Resources That Share Physical Memory

**As an** engine developer (P-26), **I want** to declare transient resources with virtual handles
and have the compiler map non-overlapping lifetimes to shared physical allocations, **so that** VRAM
usage is minimized through automatic memory sharing.

## US-2.2.3.2 Verify Memoryless Storage on Metal and Lazy Allocation on Vulkan

**As an** engine tester (P-27), **I want** to confirm that transient resources use
MTLStorageModeMemoryless on Metal and VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT on Vulkan, **so that**
tile-local attachments consume zero external memory bandwidth on mobile.

## US-2.2.4.1 Reduce Peak VRAM With Resource Aliasing

**As an** engine developer (P-26), **I want** the compiler to build an interference graph and assign
non-overlapping transient resources to the same physical memory block, **so that** peak VRAM
consumption drops significantly in complex render graphs.

## US-2.2.4.2 Validate Aliasing Across D3D12, Vulkan, and Metal

**As an** engine tester (P-27), **I want** to verify that resource aliasing uses placed resources in
heaps on D3D12, memory aliasing with explicit invalidation on Vulkan, and MTLHeap makeAliasable on
Metal, **so that** aliasing is correct per backend.

## US-2.2.5.1 Insert Minimal Barriers at the Latest Possible Point

**As an** engine developer (P-26), **I want** the graph compiler to analyze read/write sets and
insert the minimal barrier set at the latest point, using split barriers where supported,
**so that** GPU pipeline stalls are minimized.

## US-2.2.5.2 Verify Metal Emits Fences Only at Queue Boundaries

**As an** engine tester (P-27), **I want** to confirm that Metal relies on driver hazard tracking
with fences emitted only at queue boundaries, while D3D12 and Vulkan use explicit barriers,
**so that** barrier strategy is correct per backend.

## US-2.2.6.1 Overlap Async Compute With Graphics Work

**As an** engine developer (P-26), **I want** the compiler to assign passes to graphics, compute,
and copy queues based on workload type and insert cross-queue fences, **so that** async compute
passes overlap with graphics work to maximize GPU utilization.

## US-2.2.6.2 Validate Multi-Queue Assignment on Each Backend

**As an** engine tester (P-27), **I want** to verify that D3D12 and Vulkan use explicit queue
families and Metal uses shared and private command queues, **so that** multi-queue scheduling maps
correctly to each backend's queue model.

## US-2.2.7.1 Maintain Stable Pass Order Across Frames

**As an** engine developer (P-26), **I want** topological sort to produce a stable execution order
across frames for passes with no data dependency, **so that** GPU pipeline bubbles from reordering
are avoided.

## US-2.2.7.2 Verify Independent Passes Are Candidates for Parallel Encoding

**As an** engine tester (P-27), **I want** to confirm that passes with no mutual data dependency are
identified as parallelizable in the topological sort output, **so that** the parallel encoding
system (F-2.2.10) has correct input.

## US-2.2.8.1 Maintain Frame Rate by Culling Low-Priority Passes

**As a** game designer (P-5), **I want** the render graph to automatically cull lowest-priority
passes when estimated frame cost exceeds the target budget, **so that** the game maintains stable
frame rate without manual per-platform pass configuration.

## US-2.2.8.2 Validate Budget Targets Per Platform

**As an** engine tester (P-27), **I want** to verify that budget culling targets 16-33ms on mobile,
16ms docked / 33ms handheld on Switch, 16ms on desktop, and 8-16ms on high-end, **so that** pass
culling thresholds are correct per platform.

## US-2.2.9.1 Render Split-Screen, VR, and Shadow Cascades From One Graph

**As an** engine developer (P-26), **I want** a single render graph instantiated with per-view
parameter overrides for split-screen, VR stereo, shadow cascades, and reflection probes, **so that**
shared passes execute once and fan out to view-specific passes.

## US-2.2.9.2 Validate Maximum View Count Per Platform

**As an** engine tester (P-27), **I want** to verify max 4 views on mobile, 8 on Switch,
configurable dozens on desktop, and unlimited with VR stereo on high-end, **so that** multi-view
rendering stays within platform compute budgets.

## US-2.2.10.1 Encode Independent Passes on Separate Threads

**As an** engine developer (P-26), **I want** the graph compiler to distribute independent pass
encoding across a thread pool using secondary command buffers, **so that** CPU submission latency is
reduced by parallel encoding.

## US-2.2.10.2 Validate Parallel Encoding Backend Mapping

**As an** engine tester (P-27), **I want** to confirm that parallel encoding uses command list
bundles on D3D12, secondary command buffers on Vulkan, and parallel render command encoders on
Metal, **so that** encoding parallelism uses the correct native mechanism.

## US-2.2.11.1 Render With Placeholder Resources While Streaming Loads

**As a** game developer (P-15), **I want** render passes to use placeholder resources when streamed
assets are unavailable, **so that** world traversal does not stall while textures, meshes, or
acceleration structures load in the background.

## US-2.2.11.2 Validate Streaming Pool Limits Per Platform

**As an** engine tester (P-27), **I want** to verify streaming pools of 256-512MB on mobile, 1GB on
Switch, 2-4GB on desktop, and 8+GB on high-end, **so that** streaming memory stays within
per-platform budgets.

## US-2.2.12.1 Compile the Render Graph Once and Reuse Across Frames

**As an** engine developer (P-26), **I want** the full render graph compiled into a flat command
schedule once per topology change rather than every frame, **so that** per-frame parameter updates
do not trigger recompilation overhead.

## US-2.2.12.2 Verify Recompilation Triggers Only on Topology Changes

**As an** engine tester (P-27), **I want** to change pass parameters (viewport size, quality
settings) without triggering graph recompilation, and verify recompilation fires only when passes
are added or removed, **so that** recompilation frequency is minimized.

## US-2.2.13.1 Visualize the Compiled Render Graph as a DAG Overlay

**As a** technical artist (P-13), **I want** a runtime diagnostic overlay showing the compiled graph
as a DAG with per-pass GPU timing, resource lifetimes, and barrier placement, **so that** I can
identify bottleneck passes and inefficient resource usage visually.

## US-2.2.13.2 Verify Graph Diagnostics Are Stripped From Shipping Builds

**As an** engine tester (P-27), **I want** to confirm that the render graph diagnostic overlay and
logging are compile-time disabled in shipping builds, **so that** debug visualization has zero cost
in production.
