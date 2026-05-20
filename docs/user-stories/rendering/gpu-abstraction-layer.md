# User Stories -- 2.1 GPU Abstraction Layer

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.1.1.1  | engine developer (P-26)      |
| US-2.1.1.2  | engine developer (P-26)      |
| US-2.1.2.1  | engine developer (P-26)      |
| US-2.1.2.2  | engine developer (P-26)      |
| US-2.1.3.1  | engine developer (P-26)      |
| US-2.1.3.2  | technical artist (P-13)      |
| US-2.1.4.1  | engine developer (P-26)      |
| US-2.1.4.2  | engine developer (P-26)      |
| US-2.1.5.1  | engine developer (P-26)      |
| US-2.1.5.2  | engine developer (P-26)      |
| US-2.1.6.1  | engine developer (P-26)      |
| US-2.1.6.2  | engine developer (P-26)      |
| US-2.1.7.1  | engine developer (P-26)      |
| US-2.1.7.2  | engine developer (P-26)      |
| US-2.1.8.1  | engine developer (P-26)      |
| US-2.1.8.2  | game developer (P-15)        |
| US-2.1.9.1  | engine developer (P-26)      |
| US-2.1.9.2  | engine developer (P-26)      |
| US-2.1.10.1 | engine developer (P-26)      |
| US-2.1.10.2 | engine developer (P-26)      |
| US-2.1.11.1 | engine developer (P-26)      |
| US-2.1.11.2 | technical artist (P-13)      |
| US-2.1.12.1 | technical artist (P-13)      |
| US-2.1.12.2 | engine developer (P-26)      |
| US-2.1.13.1 | engine developer (P-26)      |
| US-2.1.13.2 | technical artist (P-13)      |
| US-2.1.14.1 | engine developer (P-26)      |
| US-2.1.14.2 | engine developer (P-26)      |
| US-2.1.14.3 | game developer (P-15)        |

1. **US-2.1.1.1** — **As a** engine developer (P-26), **I want** a unified GPU backend trait with
   associated types and static dispatch via generics, **so that** I can write rendering code once
   without backend-specific branching.

2. **US-2.1.1.2** — **As a** engine developer (P-26), **I want** the backend trait to eliminate
   vtable overhead at every call site, **so that** the abstraction layer costs nothing compared to
   raw API calls.

3. **US-2.1.2.1** — **As a** engine developer (P-26), **I want** a trait-based command buffer
   abstraction supporting graphics, compute, and copy operations, **so that** I can encode rendering
   work independently of the underlying API.

4. **US-2.1.2.2** — **As a** engine developer (P-26), **I want** type-safe resource binding on
   command buffers, **so that** mismatched resource types are caught at compile time rather than at
   runtime.

5. **US-2.1.3.1** — **As a** engine developer (P-26), **I want** pipeline state objects
   pre-validated at creation time, **so that** command buffer encoding incurs zero validation
   overhead during frame submission.

6. **US-2.1.3.2** — **As a** technical artist (P-13), **I want** pipeline state caches warmed at
   load time on mobile, **so that** shader variant hitches do not occur during gameplay on
   tile-based GPUs.

7. **US-2.1.4.1** — **As a** engine developer (P-26), **I want** a Vulkan backend implemented in
   Rust via `ash`, **so that** I get first-class Vulkan API access with the entire backend in Rust.

8. **US-2.1.4.2** — **As a** engine developer (P-26), **I want** the Vulkan backend built on every
   shipping platform via `ash`, **so that** rendering code is identical across Windows, macOS,
   Linux, iOS, and Android.

9. **US-2.1.5.1** — **As a** engine developer (P-26), **I want** RAII wrappers around all Vulkan
   handles, **so that** devices, queues, and resources are destroyed in the correct order.

10. **US-2.1.5.2** — **As a** engine developer (P-26), **I want** validation layers enabled
    automatically in debug builds, **so that** Vulkan API misuse is caught during development.

11. **US-2.1.6.1** — **As a** engine developer (P-26), **I want** SPIR-V shaders loaded from
    offline `naga` output, **so that** shipping builds never invoke a shader compiler at runtime.

12. **US-2.1.6.2** — **As a** engine developer (P-26), **I want** shader hot-reload to re-run
    `naga` as a subprocess when GLSL sources change, **so that** iteration does not require a full
    rebuild.

13. **US-2.1.7.1** — **As a** engine developer (P-26), **I want** a GPU heap sub-allocator that
    carves regions from pre-allocated memory blocks, **so that** thousands of per-draw uploads use
    offset binding from few OS allocations.

14. **US-2.1.7.2** — **As a** engine developer (P-26), **I want** sub-allocations to respect
    per-backend alignment requirements automatically, **so that** I do not need to track alignment
    constraints manually per platform.

15. **US-2.1.8.1** — **As a** engine developer (P-26), **I want** CPU-side shadow state tracking to
    filter redundant pipeline and binding transitions, **so that** driver overhead is minimized
    during high-frequency draws.

16. **US-2.1.8.2** — **As a** game developer (P-15), **I want** the state tracker to work
    transparently behind the command buffer API, **so that** I do not need to manually deduplicate
    state changes in gameplay code.

17. **US-2.1.9.1** — **As a** engine developer (P-26), **I want** automatic barrier batching and
    merging within command buffers, **so that** consecutive transitions on the same resource produce
    a single barrier call.

18. **US-2.1.9.2** — **As a** engine developer (P-26), **I want** split barriers inserted when
    transitions can overlap with independent work, **so that** GPU pipeline stalls are reduced.

19. **US-2.1.10.1** — **As a** engine developer (P-26), **I want** GPU work graph support enabling
    GPU-driven dispatch without CPU round-trips, **so that** culling and LOD pipelines run entirely
    on the GPU.

20. **US-2.1.10.2** — **As a** engine developer (P-26), **I want** compute-based work graph
    emulation on backends lacking native support, **so that** GPU-driven pipelines work consistently
    across Vulkan.

21. **US-2.1.11.1** — **As a** engine developer (P-26), **I want** a feature emulation layer that
    selects emulated paths at device creation time, **so that** the hot path contains no runtime
    capability branches.

22. **US-2.1.11.2** — **As a** technical artist (P-13), **I want** mesh shaders to work on older
    hardware via emulation, **so that** my meshlet-based assets render correctly regardless of GPU
    generation.

23. **US-2.1.12.1** — **As a** technical artist (P-13), **I want** per-pass GPU timestamps visible
    in the editor profiler, **so that** I can identify which rendering passes are most expensive and
    optimize content.

24. **US-2.1.12.2** — **As a** engine developer (P-26), **I want** timestamp query results read back
    one frame later, **so that** profiling does not introduce GPU stalls or pipeline bubbles.

25. **US-2.1.13.1** — **As a** engine developer (P-26), **I want** CPU work graph emulation nodes to
    appear as task graph entries in the unified game loop, **so that** GPU-driven scheduling
    integrates with CPU work.

26. **US-2.1.13.2** — **As a** technical artist (P-13), **I want** GPU-driven effects to produce
    identical results on all backends, **so that** I can author effects once and trust they work
    everywhere.

27. **US-2.1.14.1** — **As a** engine developer (P-26), **I want** all GPU resources referenced
    through generational Handle indices, **so that** use-after-free bugs are detected at runtime
    instead of causing undefined behavior.

28. **US-2.1.14.2** — **As a** engine developer (P-26), **I want** no raw GPU pointers exposed in
    any public API type, **so that** the rendering interface is memory-safe by construction.

29. **US-2.1.14.3** — **As a** game developer (P-15), **I want** type-safe handles that prevent
    passing a buffer handle where a texture handle is expected, **so that** resource type mismatches
    are caught at compile time.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.1.1 | engine developer (P-26) |
| US-2.1.10 | engine developer (P-26) |
| US-2.1.11 | engine developer (P-26) |
| US-2.1.12 | technical artist (P-13) |
| US-2.1.13 | engine developer (P-26) |
| US-2.1.14 | engine developer (P-26) |
| US-2.1.2 | engine developer (P-26) |
| US-2.1.3 | engine developer (P-26) |
| US-2.1.4 | engine developer (P-26) |
| US-2.1.5 | engine developer (P-26) |
| US-2.1.6 | engine developer (P-26) |
| US-2.1.7 | engine developer (P-26) |
| US-2.1.8 | engine developer (P-26) |
| US-2.1.9 | engine developer (P-26) |

1. **US-2.1.1** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.1.1 through US-2.1.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-2.1.10** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.10.1 through US-2.1.10.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-2.1.11** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.11.1 through US-2.1.11.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-2.1.12** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.1.12.1 through US-2.1.12.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-2.1.13** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.13.1 through US-2.1.13.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-2.1.14** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.14.1 through US-2.1.14.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-2.1.2** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.2.1 through US-2.1.2.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-2.1.3** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.3.1 through US-2.1.3.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-2.1.4** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.1.4.1 through US-2.1.4.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

10. **US-2.1.5** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-2.1.5.1 through US-2.1.5.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-2.1.6** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-2.1.6.1 through US-2.1.6.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

12. **US-2.1.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-2.1.7.1 through US-2.1.7.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-2.1.8** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-2.1.8.1 through US-2.1.8.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

14. **US-2.1.9** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-2.1.9.1 through US-2.1.9.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
