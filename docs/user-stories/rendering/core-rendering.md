# User Stories -- 2.3 Core Rendering

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.3.1.1  | environment artist (P-8)     |
| US-2.3.1.2  | engine developer (P-26)      |
| US-2.3.2.1  | environment artist (P-8)     |
| US-2.3.2.2  | engine developer (P-26)      |
| US-2.3.3.1  | engine developer (P-26)      |
| US-2.3.4.1  | game developer (P-15)        |
| US-2.3.4.2  | engine developer (P-26)      |
| US-2.3.5.1  | game developer (P-15)        |
| US-2.3.6.1  | game developer (P-15)        |
| US-2.3.7.1  | engine developer (P-26)      |
| US-2.3.7.2  | environment artist (P-8)     |
| US-2.3.8.1  | engine developer (P-26)      |
| US-2.3.8.2  | game developer (P-15)        |
| US-2.3.9.1  | environment artist (P-8)     |
| US-2.3.9.2  | engine developer (P-26)      |
| US-2.3.10.1 | effects artist (P-12)        |
| US-2.3.10.2 | game developer (P-15)        |
| US-2.3.11.1 | technical artist (P-13)      |
| US-2.3.11.2 | game developer (P-15)        |
| US-2.3.12.1 | environment artist (P-8)     |
| US-2.3.12.2 | technical artist (P-13)      |
| US-2.3.13.1 | environment artist (P-8)     |
| US-2.3.13.2 | engine developer (P-26)      |

1. **US-2.3.1.1** — **As a** environment artist (P-8), **I want** to place point, spot, and
   directional lights and see physically-based attenuation in real time, **so that** I can evaluate
   lighting composition without a bake pass.

2. **US-2.3.1.2** — **As a** engine developer (P-26), **I want** all light types written to a
   unified light buffer consumed by both forward and deferred paths, **so that** lighting is
   consistent regardless of the active pipeline.

3. **US-2.3.2.1** — **As a** environment artist (P-8), **I want** GPU frustum culling at meshlet
   granularity, **so that** dense vegetation scenes stay within the draw call budget without manual
   culling volumes.

4. **US-2.3.2.2** — **As a** engine developer (P-26), **I want** meshlet-level frustum culling via a
   compute pass testing each AABB against frustum planes, **so that** off-screen geometry is
   excluded before indirect draws.

5. **US-2.3.3.1** — **As a** engine developer (P-26), **I want** meshlet-level normal cone culling
   to skip backfacing groups before rasterization, **so that** GPU fragment workload is reduced on
   solid closed meshes.

6. **US-2.3.4.1** — **As a** game developer (P-15), **I want** two-phase HZB occlusion culling to
   reject hidden geometry automatically, **so that** interior scenes stay within GPU frame budget
   without manual occlusion volumes.

7. **US-2.3.4.2** — **As a** engine developer (P-26), **I want** phase-2 HZB retesting to recover
   newly revealed geometry within the same frame, **so that** fast camera movement never causes
   single-frame pop-in.

8. **US-2.3.5.1** — **As a** game developer (P-15), **I want** orthographic camera projection with
   adjustable width and height, **so that** I can build top-down and 2D game views without
   perspective distortion.

9. **US-2.3.6.1** — **As a** game developer (P-15), **I want** reverse-Z perspective projection with
   an infinite far plane, **so that** distant terrain has sub-pixel depth precision without
   z-fighting.

10. **US-2.3.7.1** — **As a** engine developer (P-26), **I want** GPU-driven instance compaction
    batching opaque meshlets by material into contiguous indirect draws, **so that** scenes with
    thousands of material instances render with minimal draw calls.

11. **US-2.3.7.2** — **As a** environment artist (P-8), **I want** placed props batched
    automatically by shared material, **so that** I can scatter hundreds of variants without
    managing draw call counts manually.

12. **US-2.3.8.1** — **As a** engine developer (P-26), **I want** to declare render-to-texture
    targets in the render graph with automatic barrier insertion, **so that** multi-pass effects
    require no manual synchronization.

13. **US-2.3.8.2** — **As a** game developer (P-15), **I want** to render a secondary camera to a
    texture and display it on an in-world mesh, **so that** players see security camera feeds as
    interactive elements.

14. **US-2.3.9.1** — **As a** environment artist (P-8), **I want** dynamic cubemaps that re-render
    selected faces each frame, **so that** reflections update when I move objects without a manual
    rebake.

15. **US-2.3.9.2** — **As a** engine developer (P-26), **I want** cubemap face rendering with
    seamless edge stitching, **so that** IBL prefiltering produces no visible seam artifacts.

16. **US-2.3.10.1** — **As a** effects artist (P-12), **I want** to render the scene from an
    arbitrary camera into a texture for portals and mirrors, **so that** players see a real-time
    destination view.

17. **US-2.3.10.2** — **As a** game developer (P-15), **I want** scene captures to respect
    per-platform resolution limits, **so that** captures do not exceed GPU budgets on mobile or
    Switch.

18. **US-2.3.11.1** — **As a** technical artist (P-13), **I want** to configure per-platform min/max
    render resolution with a GPU timing feedback loop, **so that** I can balance quality and
    performance per hardware tier.

19. **US-2.3.11.2** — **As a** game developer (P-15), **I want** the engine to lower internal
    resolution automatically when frame rate dips, **so that** gameplay remains responsive during
    intense scenes.

20. **US-2.3.12.1** — **As a** environment artist (P-8), **I want** to assign subsurface scattering
    profiles to skin materials and preview scatter radius in real time, **so that** I achieve
    realistic translucency without guessing parameter values.

21. **US-2.3.12.2** — **As a** technical artist (P-13), **I want** screen-space SSS to fall back to
    a preintegrated LUT on mobile, **so that** skin rendering remains acceptable across all hardware
    tiers.

22. **US-2.3.13.1** — **As a** environment artist (P-8), **I want** alpha-tested geometry to
    participate in shadow maps with a configurable threshold, **so that** fences and vegetation cast
    correct shadows.

23. **US-2.3.13.2** — **As a** engine developer (P-26), **I want** distant alpha-tested vegetation
    to fall back to opaque proxies on tile-based mobile GPUs, **so that** alpha cutouts do not break
    hidden surface removal.
