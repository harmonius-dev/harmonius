# Profiling and Collaboration

Performance analysis, debugging, and multiplayer development.

## What it covers

- CPU profiler: measuring per-function execution time.
- GPU profiler: measuring GPU workload and bottlenecks.
- Memory profiler: tracking allocations and leaks.
- Frame analysis: per-frame performance breakdown.
- Network profiler: measuring bandwidth and latency.
- Frame debugging: inspecting per-frame GPU state and draws.
- Collaborative editing: multiple developers working simultaneously.
- Version control: tracking asset and code changes.
- Code review: peer review and feedback.
- Performance budgets: tracking against targets.

## Concepts

### CPU Profiling

CPU profiler measures per-function execution time. Instrumentation code wraps functions,
recording entry and exit times. Call trees show function hierarchy: main loop calls physics system,
which calls collision detection. Profiler identifies bottlenecks: if physics takes 8ms/frame
(budget 5ms), optimize physics. Per-frame breakdown shows: rendering 8ms, physics 7ms, AI 3ms, etc.

### GPU Profiling

GPU profiler intercepts GPU commands, measuring per-draw and per-pass time. GPU capture tools
(RenderDoc, Nvidia NSight) show all GPU work. Bottleneck analysis: if 60% time is shaders, optimize
shaders; if 60% time is texture bandwidth, reduce texture size. VRAM profiler tracks texture and
buffer memory usage.

### Memory Profiling

Memory profiler tracks allocations: when allocated, how much, by whom. Leak detection identifies
allocations never freed. Live memory graphs show memory over time. Fragmentation analysis identifies
excessive small allocations causing fragmentation.

### Collaborative Editing

Multiple developers edit scenes simultaneously via locks or branching. Scene merging combines
changes from different developers. Conflict resolution: both edit entity position; last-write-wins
or manual merge. Collaborative version control tools (Perforce, Plastic SCM) support binary assets.

### Frame Debugging

Frame debugging replays a frame with full GPU state capture. Inspect every draw call: geometry,
textures, shader parameters. Visual debugging overlays: red wireframe, overdraw visualization.
GPU debuggers allow stepping through shaders, inspecting GPU register values.

## How it fits

- See [editor-framework.md](./editor-framework.md) for editor integration.
- See [cloud-and-deployment.md](./cloud-and-deployment.md) for cloud-based collaboration and
  metrics.
- See [../core-runtime/simulation-loop.md](../core-runtime/simulation-loop.md) for frame
  structure and timing.
