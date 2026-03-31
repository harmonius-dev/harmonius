# User Stories -- 2.2 Render Graph

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.2.1.1  | engine developer (P-26)      |
| US-2.2.1.2  | game developer (P-15)        |
| US-2.2.2.1  | engine developer (P-26)      |
| US-2.2.2.2  | technical artist (P-13)      |
| US-2.2.3.1  | engine developer (P-26)      |
| US-2.2.3.2  | engine developer (P-26)      |
| US-2.2.4.1  | engine developer (P-26)      |
| US-2.2.5.1  | engine developer (P-26)      |
| US-2.2.5.2  | game developer (P-15)        |
| US-2.2.6.1  | engine developer (P-26)      |
| US-2.2.6.2  | engine developer (P-26)      |
| US-2.2.7.1  | engine developer (P-26)      |
| US-2.2.8.1  | technical artist (P-13)      |
| US-2.2.8.2  | engine developer (P-26)      |
| US-2.2.9.1  | game developer (P-15)        |
| US-2.2.9.2  | engine developer (P-26)      |
| US-2.2.10.1 | engine developer (P-26)      |
| US-2.2.11.1 | environment artist (P-8)     |
| US-2.2.11.2 | engine developer (P-26)      |
| US-2.2.12.1 | engine developer (P-26)      |
| US-2.2.13.1 | technical artist (P-13)      |
| US-2.2.13.2 | engine developer (P-26)      |
| US-2.2.14.1 | engine developer (P-26)      |
| US-2.2.15.1 | engine developer (P-26)      |

1. **US-2.2.1.1** — **As a** engine developer (P-26), **I want** to declare render passes as structs
   implementing a pass trait with resource read/write sets, **so that** the graph compiler discovers
   dependency topology automatically.

2. **US-2.2.1.2** — **As a** game developer (P-15), **I want** to register a custom render pass
   without manually ordering it relative to built-in passes, **so that** I can extend the pipeline
   without fragile ordering code.

3. **US-2.2.2.1** — **As a** engine developer (P-26), **I want** each pass to declare required GPU
   capabilities, **so that** the graph compiler prunes passes exceeding the device feature set and
   falls back to alternatives.

4. **US-2.2.2.2** — **As a** technical artist (P-13), **I want** ray tracing passes to degrade
   gracefully on hardware without RT support, **so that** my scenes render acceptably on all target
   platforms.

5. **US-2.2.3.1** — **As a** engine developer (P-26), **I want** passes to declare virtual resource
   handles with format and dimensions, **so that** the compiler maps them to physical allocations
   with lifetime-based sharing.

6. **US-2.2.3.2** — **As a** engine developer (P-26), **I want** transient resources on mobile to
   use memoryless storage automatically, **so that** tile-local attachments avoid external memory
   bandwidth.

7. **US-2.2.4.1** — **As a** engine developer (P-26), **I want** automatic memory aliasing of
   non-overlapping transient resources, **so that** peak VRAM consumption is minimized without
   manual resource management.

8. **US-2.2.5.1** — **As a** engine developer (P-26), **I want** the graph compiler to insert
   minimal barriers between passes based on resource read/write analysis, **so that** I never need
   to manage synchronization manually.

9. **US-2.2.5.2** — **As a** game developer (P-15), **I want** barriers placed automatically when I
   chain render-to-texture passes, **so that** read-after-write hazards are impossible in my custom
   effects.

10. **US-2.2.6.1** — **As a** engine developer (P-26), **I want** the compiler to assign passes to
    graphics, compute, and copy queues based on workload type, **so that** async compute overlaps
    with graphics work.

11. **US-2.2.6.2** — **As a** engine developer (P-26), **I want** cross-queue synchronization fences
    inserted automatically, **so that** multi-queue scheduling is correct without manual fence
    management.

12. **US-2.2.7.1** — **As a** engine developer (P-26), **I want** passes topologically sorted by
    resource dependencies with stable ordering across frames, **so that** GPU pipeline bubbles from
    reordering are avoided.

13. **US-2.2.8.1** — **As a** technical artist (P-13), **I want** the graph compiler to cull
    low-priority passes when the frame budget is exceeded, **so that** visual quality degrades
    gracefully under load.

14. **US-2.2.8.2** — **As a** engine developer (P-26), **I want** per-pass cost estimates driven by
    historical GPU timing data, **so that** budget culling decisions reflect actual hardware
    performance.

15. **US-2.2.9.1** — **As a** game developer (P-15), **I want** a single render graph instantiated
    for split-screen, VR stereo, and shadow cascade views, **so that** shared passes execute once
    and fan out to per-view passes.

16. **US-2.2.9.2** — **As a** engine developer (P-26), **I want** multi-view execution to share
    culling and lighting computation across views, **so that** per-view overhead scales
    sub-linearly.

17. **US-2.2.10.1** — **As a** engine developer (P-26), **I want** independent passes encoded on
    separate threads using secondary command buffers, **so that** CPU submission latency is reduced
    through parallel encoding.

18. **US-2.2.11.1** — **As a** environment artist (P-8), **I want** passes depending on streamed
    textures to use placeholders until streaming completes, **so that** world traversal does not
    stall on missing assets.

19. **US-2.2.11.2** — **As a** engine developer (P-26), **I want** the graph compiler to substitute
    placeholder resources or skip dependent passes for unavailable assets, **so that** streaming
    never causes GPU idle time.

20. **US-2.2.12.1** — **As a** engine developer (P-26), **I want** the render graph compiled once
    and reused until the pass set or device capabilities change, **so that** per-frame recompilation
    overhead is eliminated.

21. **US-2.2.13.1** — **As a** technical artist (P-13), **I want** a diagnostic overlay visualizing
    the compiled graph with per-pass GPU timing and resource lifetimes, **so that** I can identify
    bottlenecks in the pipeline.

22. **US-2.2.13.2** — **As a** engine developer (P-26), **I want** to export captured frame graphs
    for offline analysis, **so that** I can debug rendering issues that are hard to reproduce in
    real time.

23. **US-2.2.14.1** — **As a** engine developer (P-26), **I want** render passes to compile into
    task graph nodes alongside ECS, physics, and audio phases, **so that** the scheduler overlaps
    CPU prep with GPU submission.

24. **US-2.2.15.1** — **As a** engine developer (P-26), **I want** GPU command buffers borrowed via
    scoped APIs enforcing encoding lifetime at compile time, **so that** use-after-submit bugs are
    impossible.
