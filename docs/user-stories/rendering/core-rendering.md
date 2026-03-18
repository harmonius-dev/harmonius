# User Stories -- 2.3 Core Rendering

## Stories

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-2.3.1.1  | environment artist (P-8) |          |              |
| US-2.3.1.2  | engine tester (P-27)     |          |              |
| US-2.3.1.3  | engine developer (P-26)  |          |              |
| US-2.3.2.1  | environment artist (P-8) |          |              |
| US-2.3.2.2  | engine tester (P-27)     |          |              |
| US-2.3.3.1  | engine developer (P-26)  |          |              |
| US-2.3.3.2  | engine tester (P-27)     |          |              |
| US-2.3.4.1  | game developer (P-15)    |          |              |
| US-2.3.4.2  | engine tester (P-27)     |          |              |
| US-2.3.4.3  | engine developer (P-26)  |          |              |
| US-2.3.5.1  | game designer (P-5)      |          |              |
| US-2.3.5.2  | engine tester (P-27)     |          |              |
| US-2.3.6.1  | game developer (P-15)    |          |              |
| US-2.3.6.2  | engine tester (P-27)     |          |              |
| US-2.3.7.1  | engine developer (P-26)  |          |              |
| US-2.3.7.2  | environment artist (P-8) |          |              |
| US-2.3.7.3  | engine tester (P-27)     |          |              |
| US-2.3.8.1  | engine developer (P-26)  |          |              |
| US-2.3.8.2  | game designer (P-5)      |          |              |
| US-2.3.9.1  | environment artist (P-8) |          |              |
| US-2.3.9.2  | engine tester (P-27)     |          |              |
| US-2.3.10.1 | effects artist (P-12)    |          |              |
| US-2.3.10.2 | engine tester (P-27)     |          |              |
| US-2.3.11.1 | player (P-23)            |          |              |
| US-2.3.11.2 | technical artist (P-13)  |          |              |
| US-2.3.11.3 | engine tester (P-27)     |          |              |
| US-2.3.12.1 | character artist (P-9)   |          |              |
| US-2.3.12.2 | engine tester (P-27)     |          |              |
| US-2.3.13.1 | environment artist (P-8) |          |              |
| US-2.3.13.2 | engine developer (P-26)  |          |              |

1. **US-2.3.1.1** — I want to place point, spot, and directional lights in a scene and see
   physically-based attenuation in real time
   - **Acceptance:** I can evaluate lighting composition and color without waiting for a bake pass
2. **US-2.3.1.2** — I want to run automated comparison tests of the unified light buffer output
   between the forward and deferred rendering paths
   - **Acceptance:** I can verify that both paths produce identical lighting results across all
     light types
3. **US-2.3.1.3** — I want to query per-tile light assignment counts during a stress test with 256
   overlapping lights
   - **Acceptance:** I can detect tile overflow and validate platform-specific tile caps (16 on
     mobile, 32 on Switch, 256 on desktop)
4. **US-2.3.2.1** — I want GPU frustum culling at meshlet granularity to automatically exclude
   off-screen foliage
   - **Acceptance:** I can build dense vegetation scenes without worrying about manual culling
     volumes or draw call budgets
5. **US-2.3.2.2** — I want to sweep the camera through a fully populated test scene at 1-degree
   increments and compare against a brute-force reference render
   - **Acceptance:** I can confirm no meshlets are incorrectly culled near frustum boundaries
6. **US-2.3.3.1** — I want meshlet-level normal cone culling to skip backfacing meshlet groups
   before rasterization
   - **Acceptance:** GPU fragment workload is reduced by 30-50% on solid closed meshes
7. **US-2.3.3.2** — I want to verify that meshlets flagged as two-sided (foliage, curtains) are
   excluded from normal cone culling
   - **Acceptance:** double-sided materials render correctly from all viewing angles
8. **US-2.3.4.1** — I want two-phase HZB occlusion culling to automatically reject geometry hidden
   behind walls and furniture
   - **Acceptance:** interior scenes with heavy overdraw stay within the GPU frame budget without
     manual occlusion volumes
9. **US-2.3.4.2** — I want to record a fast camera pan that reveals previously occluded geometry and
   verify that phase-2 HZB retesting renders it within the same frame
   - **Acceptance:** players never see single-frame pop-in when turning quickly
10. **US-2.3.4.3** — I want to visualize the HZB mip chain as a buffer overlay alongside the final
    depth buffer
    - **Acceptance:** I can diagnose conservative occlusion errors when objects are incorrectly
      culled at distance
11. **US-2.3.5.1** — I want to configure an orthographic projection camera with adjustable width and
    height
    - **Acceptance:** I can preview and iterate on a top-down game layout without perspective
      distortion
12. **US-2.3.5.2** — I want to render shadow maps using orthographic projection and compare shadow
    coverage against a known reference
    - **Acceptance:** directional light shadow cascades produce correct results in orthographic
      views
13. **US-2.3.6.1** — I want reverse-Z perspective projection with an infinite far plane
    - **Acceptance:** distant terrain and skybox elements have sub-pixel depth precision without
      z-fighting artifacts
14. **US-2.3.6.2** — I want to run depth precision tests on Metal, D3D12, and Vulkan to verify that
    reverse-Z clears depth to 0 correctly on each backend
    - **Acceptance:** depth comparison produces identical results across platforms
15. **US-2.3.7.1** — I want GPU-driven instance compaction to batch surviving opaque meshlets by
    material into contiguous indirect draws
    - **Acceptance:** a scene with 10,000 material instances renders with fewer than 100 draw calls
16. **US-2.3.7.2** — I want the instancing system to automatically batch my placed props by shared
    material
    - **Acceptance:** I can scatter hundreds of unique asset variants across a level without
      manually managing draw call counts
17. **US-2.3.7.3** — I want to place overlapping transparent objects at various depths and confirm
    they render in back-to-front sorted order
    - **Acceptance:** transparency compositing is visually correct without sorting artifacts
18. **US-2.3.8.1** — I want to declare a render-to-texture target in the render graph and have
    barriers inserted automatically between write and read
    - **Acceptance:** I can chain multi-pass effects without manual synchronization
19. **US-2.3.8.2** — I want to render a secondary camera view to a texture and display it on an
    in-world monitor mesh
    - **Acceptance:** players can see security camera feeds as interactive gameplay elements
20. **US-2.3.9.1** — I want to place a reflection probe that re-renders selected cubemap faces each
    frame
    - **Acceptance:** reflections update dynamically when I move objects in the scene without a
      manual rebake
21. **US-2.3.9.2** — I want to render a test scene to all six cubemap faces and verify seamless
    stitching at face edges
    - **Acceptance:** cubemap-based reflections and IBL prefiltering have no visible seam artifacts
22. **US-2.3.10.1** — I want to render the scene from a second camera into a texture and apply it to
    a portal mesh with distortion
    - **Acceptance:** players see a real-time view of the destination through the portal surface
23. **US-2.3.10.2** — I want to confirm that scene captures respect platform limits
    (quarter-resolution on mobile, half on Switch, configurable on desktop)
    - **Acceptance:** scene captures do not exceed per-platform GPU budgets
24. **US-2.3.11.1** — I want the game to automatically lower its internal resolution when the frame
    rate dips during intense combat
    - **Acceptance:** gameplay remains responsive without visible stuttering
25. **US-2.3.11.2** — I want to configure per-platform minimum and maximum render resolution
    percentages with a GPU timing feedback loop
    - **Acceptance:** I can balance visual quality and performance targets for each hardware tier
26. **US-2.3.11.3** — I want to run a frame-time stress test and verify that dynamic resolution
    converges to a stable percentage without oscillating
    - **Acceptance:** players do not experience distracting resolution flickering
27. **US-2.3.12.1** — I want to assign a subsurface scattering profile to skin materials and preview
    the scatter radius in real time
    - **Acceptance:** I can achieve realistic skin translucency for hero characters without guessing
      at parameter values
28. **US-2.3.12.2** — I want to compare screen-space SSS output on desktop against the preintegrated
    LUT fallback on mobile
    - **Acceptance:** skin rendering remains visually acceptable across all hardware tiers
29. **US-2.3.13.1** — I want alpha-tested geometry to participate in shadow map rendering with a
    configurable alpha threshold
    - **Acceptance:** fences and vegetation cast correct shadows without manual shadow proxy meshes
30. **US-2.3.13.2** — I want to profile alpha-test overdraw on tile-based mobile GPUs and verify
    that distant vegetation falls back to opaque proxies
    - **Acceptance:** alpha cutouts do not break hidden surface removal and cause bandwidth spikes
      on mobile
