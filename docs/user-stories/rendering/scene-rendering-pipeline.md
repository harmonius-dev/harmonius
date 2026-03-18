# User Stories -- 2.10 Scene Rendering Pipeline

## Stories

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-2.10.1.1  | engine developer (P-26) |          |              |
| US-2.10.1.2  | engine tester (P-27)    |          |              |
| US-2.10.2.1  | engine developer (P-26) |          |              |
| US-2.10.2.2  | engine developer (P-26) |          |              |
| US-2.10.3.1  | engine developer (P-26) |          |              |
| US-2.10.3.2  | engine tester (P-27)    |          |              |
| US-2.10.3.3  | engine developer (P-26) |          |              |
| US-2.10.4.1  | game developer (P-15)   |          |              |
| US-2.10.4.2  | engine tester (P-27)    |          |              |
| US-2.10.5.1  | engine developer (P-26) |          |              |
| US-2.10.5.2  | engine tester (P-27)    |          |              |
| US-2.10.6.1  | engine developer (P-26) |          |              |
| US-2.10.6.2  | engine tester (P-27)    |          |              |
| US-2.10.7.1  | engine developer (P-26) |          |              |
| US-2.10.7.2  | engine tester (P-27)    |          |              |
| US-2.10.8.1  | engine developer (P-26) |          |              |
| US-2.10.8.2  | engine tester (P-27)    |          |              |
| US-2.10.9.1  | game developer (P-15)   |          |              |
| US-2.10.9.2  | engine tester (P-27)    |          |              |
| US-2.10.10.1 | technical artist (P-13) |          |              |
| US-2.10.10.2 | engine tester (P-27)    |          |              |

1. **US-2.10.1.1** — I want render proxy extraction to run on a dedicated thread reading ECS
   components via immutable queries
   - **Acceptance:** simulation advances concurrently with rendering without contention
2. **US-2.10.1.2** — I want to run a stress test where simulation mutates entities while extraction
   reads, and verify no data races occur
   - **Acceptance:** concurrent extraction is safe across all platforms
3. **US-2.10.2.1** — I want render-side proxy components (mesh, material, transform, bounds) stored
   in structure-of-arrays layout
   - **Acceptance:** GPU upload and iteration are cache-friendly and simulation-only fields are
     discarded
4. **US-2.10.2.2** — I want to benchmark render proxy iteration in SoA layout against an AoS
   baseline at 100K entities
   - **Acceptance:** I can verify the cache performance benefit is measurable
5. **US-2.10.3.1** — I want dirty-flag change detection to drive incremental proxy updates so only
   entities with changed transform, material, or mesh are re-uploaded
   - **Acceptance:** per-frame CPU and bus bandwidth is O(changed) rather than O(total)
6. **US-2.10.3.2** — I want to spawn 50K entities, move 100 of them, and verify that exactly 100
   proxies are re-uploaded while the remaining 49,900 are unchanged
   - **Acceptance:** incremental update produces correct results at scale
7. **US-2.10.3.3** — I want to profile bus bandwidth for incremental proxy updates on mobile
   LPDDR4/5 shared memory
   - **Acceptance:** I can confirm change detection reduces bandwidth to a level appropriate for
     mobile's limited bus
8. **US-2.10.4.1** — I want to register main camera, shadow cascade, reflection probe, and
   split-screen player views each with their own projection, viewport, and quality tier
   - **Acceptance:** multi-view rendering is set up cleanly without manual duplication
9. **US-2.10.4.2** — I want to register 4 views and confirm that each produces independent culling
   and draw passes while sharing extracted proxy data
   - **Acceptance:** multi-view execution works as specified
10. **US-2.10.5.1** — I want simultaneous rendering of many views (shadow cascades, reflection
    probes, player cameras) from a single extracted snapshot
    - **Acceptance:** MMO scenes with many shadow-casting lights render without per-view extraction
      overhead
11. **US-2.10.5.2** — I want to confirm that VR stereo views use single-pass instanced rendering
    where the backend supports viewport instancing
    - **Acceptance:** VR rendering does not double the geometry workload
12. **US-2.10.6.1** — I want draw lists keyed by material, mesh, and render state with sort keys
    encoding pipeline state, material ID, and depth
    - **Acceptance:** GPU state changes are minimized during submission
13. **US-2.10.6.2** — I want to measure draw submission time on mobile with and without sort key
    optimization and verify that sorted submission reduces driver overhead
    - **Acceptance:** mobile scenes benefit from draw list ordering
14. **US-2.10.7.1** — I want a GPU compute pass to compact surviving draws into contiguous indirect
    draw buffers grouped by material
    - **Acceptance:** hundreds of thousands of meshlet instances render with minimal draw calls and
      zero per-draw CPU dispatch
15. **US-2.10.7.2** — I want to confirm that GPU batch compaction requires Vulkan 1.1+ or Metal GPU
    family 3+ on mobile and is correctly gated
    - **Acceptance:** the feature activates only on mobile devices with indirect draw support
16. **US-2.10.8.1** — I want per-draw material parameters (textures, constants, samplers) bound via
    bindless descriptor indices in a per-instance buffer
    - **Acceptance:** descriptor set switching is eliminated and material-agnostic batching is
      possible
17. **US-2.10.8.2** — I want to verify that bindless uses argument buffers on Metal, root descriptor
    tables on D3D12, and VK_EXT_descriptor_indexing on Vulkan
    - **Acceptance:** bindless material access works correctly on each backend
18. **US-2.10.9.1** — I want an immediate-mode debug drawing API for lines, wireframe shapes, text
    labels, and gizmos rendered as an overlay pass
    - **Acceptance:** I can visualize collision volumes, nav meshes, and game state during
      development
19. **US-2.10.9.2** — I want to confirm that debug primitives are compile-time gated and absent from
    shipping builds
    - **Acceptance:** debug visualization has zero cost in production
20. **US-2.10.10.1** — I want to select diagnostic render modes (depth, normals, motion vectors,
    roughness, metallic, overdraw heat maps) from a debug menu
    - **Acceptance:** I can inspect intermediate buffers to diagnose rendering issues
21. **US-2.10.10.2** — I want to confirm that G-buffer visualization modes are unavailable on mobile
    when the deferred path is disabled
    - **Acceptance:** diagnostic modes do not offer inaccessible buffer views
