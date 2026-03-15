# User Stories -- 2.10 Scene Rendering Pipeline

## US-2.10.1.1 Extract Renderable Entities Without Blocking Simulation

**As an** engine developer (P-26), **I want** render proxy extraction to run on a dedicated thread
reading ECS components via immutable queries, **so that** simulation advances concurrently with
rendering without contention.

## US-2.10.1.2 Validate Extraction Thread Safety Under Concurrent Write

**As an** engine tester (P-27), **I want** to run a stress test where simulation mutates entities
while extraction reads, and verify no data races occur, **so that** concurrent extraction is safe
across all platforms.

## US-2.10.2.1 Store Render Proxies in Cache-Friendly SoA Layout

**As an** engine developer (P-26), **I want** render-side proxy components (mesh, material,
transform, bounds) stored in structure-of-arrays layout, **so that** GPU upload and iteration are
cache-friendly and simulation-only fields are discarded.

## US-2.10.2.2 Profile SoA Iteration Performance vs AoS Baseline

**As an** engine developer (P-26), **I want** to benchmark render proxy iteration in SoA layout
against an AoS baseline at 100K entities, **so that** I can verify the cache performance benefit is
measurable.

## US-2.10.3.1 Upload Only Changed Entities Each Frame

**As an** engine developer (P-26), **I want** dirty-flag change detection to drive incremental proxy
updates so only entities with changed transform, material, or mesh are re-uploaded, **so that**
per-frame CPU and bus bandwidth is O(changed) rather than O(total).

## US-2.10.3.2 Validate Incremental Update Correctness at MMO Scale

**As an** engine tester (P-27), **I want** to spawn 50K entities, move 100 of them, and verify that
exactly 100 proxies are re-uploaded while the remaining 49,900 are unchanged, **so that**
incremental update produces correct results at scale.

## US-2.10.3.3 Measure Bus Bandwidth Savings on Mobile LPDDR

**As an** engine developer (P-26), **I want** to profile bus bandwidth for incremental proxy updates
on mobile LPDDR4/5 shared memory, **so that** I can confirm change detection reduces bandwidth to a
level appropriate for mobile's limited bus.

## US-2.10.4.1 Register Multiple Camera Views for Split-Screen

**As a** game developer (P-15), **I want** to register main camera, shadow cascade, reflection
probe, and split-screen player views each with their own projection, viewport, and quality tier,
**so that** multi-view rendering is set up cleanly without manual duplication.

## US-2.10.4.2 Verify View Registration Feeds Multi-View Execution

**As an** engine tester (P-27), **I want** to register 4 views and confirm that each produces
independent culling and draw passes while sharing extracted proxy data, **so that** multi-view
execution works as specified.

## US-2.10.5.1 Render Dozens of Concurrent Views for MMO Shadow Lights

**As an** engine developer (P-26), **I want** simultaneous rendering of many views (shadow cascades,
reflection probes, player cameras) from a single extracted snapshot, **so that** MMO scenes with
many shadow-casting lights render without per-view extraction overhead.

## US-2.10.5.2 Validate VR Stereo Single-Pass Instanced Rendering

**As an** engine tester (P-27), **I want** to confirm that VR stereo views use single-pass instanced
rendering where the backend supports viewport instancing, **so that** VR rendering does not double
the geometry workload.

## US-2.10.6.1 Build Sorted Draw Lists With Minimal State Changes

**As an** engine developer (P-26), **I want** draw lists keyed by material, mesh, and render state
with sort keys encoding pipeline state, material ID, and depth, **so that** GPU state changes are
minimized during submission.

## US-2.10.6.2 Validate Sort Key Order Is Critical on Mobile

**As an** engine tester (P-27), **I want** to measure draw submission time on mobile with and
without sort key optimization and verify that sorted submission reduces driver overhead, **so that**
mobile scenes benefit from draw list ordering.

## US-2.10.7.1 Compact Post-Cull Draws Into Indirect Buffers on GPU

**As an** engine developer (P-26), **I want** a GPU compute pass to compact surviving draws into
contiguous indirect draw buffers grouped by material, **so that** hundreds of thousands of meshlet
instances render with minimal draw calls and zero per-draw CPU dispatch.

## US-2.10.7.2 Validate Indirect Draw Support on Mobile Vulkan and Metal

**As an** engine tester (P-27), **I want** to confirm that GPU batch compaction requires Vulkan 1.1+
or Metal GPU family 3+ on mobile and is correctly gated, **so that** the feature activates only on
mobile devices with indirect draw support.

## US-2.10.8.1 Bind Material Parameters via Bindless Descriptors

**As an** engine developer (P-26), **I want** per-draw material parameters (textures, constants,
samplers) bound via bindless descriptor indices in a per-instance buffer, **so that** descriptor set
switching is eliminated and material-agnostic batching is possible.

## US-2.10.8.2 Validate Bindless Binding Across Metal, D3D12, and Vulkan

**As an** engine tester (P-27), **I want** to verify that bindless uses argument buffers on Metal,
root descriptor tables on D3D12, and VK_EXT_descriptor_indexing on Vulkan, **so that** bindless
material access works correctly on each backend.

## US-2.10.9.1 Draw Debug Lines, Wireframes, and Text Labels at Runtime

**As a** game developer (P-15), **I want** an immediate-mode debug drawing API for lines, wireframe
shapes, text labels, and gizmos rendered as an overlay pass, **so that** I can visualize collision
volumes, nav meshes, and game state during development.

## US-2.10.9.2 Verify Debug Drawing Is Stripped From Shipping Builds

**As an** engine tester (P-27), **I want** to confirm that debug primitives are compile-time gated
and absent from shipping builds, **so that** debug visualization has zero cost in production.

## US-2.10.10.1 Switch to Buffer Visualization Mode for Debugging

**As a** technical artist (P-13), **I want** to select diagnostic render modes (depth, normals,
motion vectors, roughness, metallic, overdraw heat maps) from a debug menu, **so that** I can
inspect intermediate buffers to diagnose rendering issues.

## US-2.10.10.2 Validate G-Buffer Visualization Unavailable on Mobile When Deferred Disabled

**As an** engine tester (P-27), **I want** to confirm that G-buffer visualization modes are
unavailable on mobile when the deferred path is disabled, **so that** diagnostic modes do not offer
inaccessible buffer views.
