# User Stories -- 2.10 Scene Rendering Pipeline

## Stories

| ID           | Persona                      |
|--------------|------------------------------|
| US-2.10.1.1  | engine developer (P-26)      |
| US-2.10.1.2  | technical artist (P-13)      |
| US-2.10.2.1  | engine developer (P-26)      |
| US-2.10.3.1  | engine developer (P-26)      |
| US-2.10.3.2  | technical artist (P-13)      |
| US-2.10.4.1  | game developer (P-15)        |
| US-2.10.4.2  | engine developer (P-26)      |
| US-2.10.5.1  | game developer (P-15)        |
| US-2.10.5.2  | engine developer (P-26)      |
| US-2.10.6.1  | engine developer (P-26)      |
| US-2.10.6.2  | technical artist (P-13)      |
| US-2.10.7.1  | engine developer (P-26)      |
| US-2.10.7.2  | technical artist (P-13)      |
| US-2.10.8.1  | engine developer (P-26)      |
| US-2.10.8.2  | technical artist (P-13)      |
| US-2.10.9.1  | engine developer (P-26)      |
| US-2.10.9.2  | game developer (P-15)        |
| US-2.10.10.1 | technical artist (P-13)      |
| US-2.10.10.2 | engine developer (P-26)      |

1. **US-2.10.1.1** — **As a** engine developer (P-26), **I want** visible ECS entities extracted
   into a renderer-owned snapshot each frame on a dedicated thread, **so that** simulation advances
   concurrently with rendering.

2. **US-2.10.1.2** — **As a** technical artist (P-13), **I want** extraction to use immutable
   queries, **so that** reading component data for rendering never blocks gameplay systems.

3. **US-2.10.2.1** — **As a** engine developer (P-26), **I want** lightweight render proxies in a
   flat SoA layout holding only GPU-needed data, **so that** upload and iteration are
   cache-friendly.

4. **US-2.10.3.1** — **As a** engine developer (P-26), **I want** dirty flags driving incremental
   proxy updates, **so that** only changed entities re-upload per frame and CPU-bus bandwidth is
   O(changed) not O(N).

5. **US-2.10.3.2** — **As a** technical artist (P-13), **I want** incremental extraction to be
   transparent, **so that** I do not need to manually mark entities as dirty when editing materials
   or transforms.

6. **US-2.10.4.1** — **As a** game developer (P-15), **I want** each active view registered with
   projection, view matrix, viewport rect, and quality tier, **so that** split-screen and VR views
   are first-class render inputs.

7. **US-2.10.4.2** — **As a** engine developer (P-26), **I want** views as typed inputs to the
   render graph multi-view execution, **so that** shadow cascades and reflection probes share the
   same pipeline.

8. **US-2.10.5.1** — **As a** game developer (P-15), **I want** simultaneous rendering of many
   independent views from one extracted snapshot, **so that** split-screen and shadow views share
   proxy data.

9. **US-2.10.5.2** — **As a** engine developer (P-26), **I want** VR stereo views to use single-pass
   instanced rendering where the backend supports viewport instancing, **so that** stereo overhead
   is minimal.

10. **US-2.10.6.1** — **As a** engine developer (P-26), **I want** per-view draw lists sorted by
    material, mesh, and render state to minimize state changes, **so that** GPU submission is
    efficient.

11. **US-2.10.6.2** — **As a** technical artist (P-13), **I want** sort key order optimized for
    mobile driver overhead, **so that** state changes are minimized on platforms with higher
    per-change cost.

12. **US-2.10.7.1** — **As a** engine developer (P-26), **I want** a GPU compute pass compacting
    post-cull draws into contiguous indirect buffers grouped by material, **so that** per-draw CPU
    dispatch overhead is eliminated.

13. **US-2.10.7.2** — **As a** technical artist (P-13), **I want** GPU batch compaction to render
    hundreds of thousands of meshlet instances with minimal draw calls, **so that** dense scenes
    stay within CPU submission budget.

14. **US-2.10.8.1** — **As a** engine developer (P-26), **I want** per-draw material parameters
    bound via bindless descriptor indices in a per-instance buffer, **so that** descriptor set
    switching is eliminated between draws.

15. **US-2.10.8.2** — **As a** technical artist (P-13), **I want** material-agnostic batching via
    bindless binding, **so that** diverse material sets do not increase draw call count.

16. **US-2.10.9.1** — **As a** engine developer (P-26), **I want** an immediate-mode debug drawing
    API for lines, wireframes, and text labels rendered as an overlay, **so that** I can visualize
    physics shapes and spatial indices at runtime.

17. **US-2.10.9.2** — **As a** game developer (P-15), **I want** debug gizmos compiled out of
    shipping builds, **so that** debug visualization has zero runtime cost in release.

18. **US-2.10.10.1** — **As a** technical artist (P-13), **I want** diagnostic render modes showing
    depth, normals, roughness, metallic, and overdraw heat maps, **so that** I can inspect
    intermediate buffers to diagnose issues.

19. **US-2.10.10.2** — **As a** engine developer (P-26), **I want** buffer visualization stripped
    from shipping builds, **so that** debug modes have no overhead in release.
