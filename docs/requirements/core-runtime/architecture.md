# Architecture Requirements

## R-1.1.1 Safe User-Facing API

The user-facing API SHALL be 100% safe C++. The safe subset prohibits raw pointer arithmetic, manual
memory management, C-style casts, and unchecked array access. Unsafe operations SHALL be confined to
background render and IO threads behind clearly marked internal boundaries. This guarantees that user
code cannot trigger undefined behavior.

## R-1.1.2 GPU-Driven Rendering

All scene rendering SHALL be GPU-driven. Draw calls for scene geometry SHALL be dispatched by the GPU
via indirect dispatch buffers populated by compute culling passes. Direct CPU-side draw call
submission SHALL be permitted only for operations that cannot be expressed as indirect dispatches,
such as async compute dispatches and mesh shader launches. This eliminates CPU draw-call bottlenecks
for scene rendering while preserving direct dispatch where indirect dispatch is unnecessary or
unsupported.

## R-1.1.3 Mesh Shader Pipeline

Mesh shaders (task/object + mesh stages) SHALL be the sole geometry pipeline. No traditional
vertex/geometry/tessellation pipeline SHALL be supported. This reduces code surface and enables
meshlet-level GPU culling.

## R-1.1.4 Declarative Render Graph

The rendering pipeline SHALL be defined as a declarative, frame-invariant directed acyclic graph of
passes and resources. The graph SHALL be compiled once into an optimized execution plan and
re-executed each frame with only per-frame data changing. This enables automatic barrier insertion,
resource aliasing, and pass scheduling.

## R-1.1.5 Cross-Platform via Native Backends

Each GPU API SHALL be supported through a native backend. No compatibility/translation layers
(MoltenVK, DXVK, etc.) SHALL be used. Direct native API access ensures maximum performance and
avoids translation overhead. One backend SHALL be selected at build time via CMake and statically
linked -- all dispatch SHALL be static (compile-time polymorphism via C++20 concepts). No virtual
methods, no vtables, no dynamic dispatch, no shared libraries SHALL be used in any backend or
platform abstraction layer.

## R-1.1.6 Modern Hardware Only

The system SHALL require bindless resources, mesh shaders, and hardware ray tracing (for RT
features). No legacy fallback paths SHALL exist. This keeps the codebase lean and avoids maintaining
parallel code paths for deprecated hardware.

## R-1.1.7 Strict Layer Separation

The system SHALL enforce strict three-layer separation: GPU backend (`harmonius::gpu`), GPU runtime
(`harmonius::gpu_runtime`), and render graph (`harmonius::rg`). The GPU runtime layer SHALL depend
only on the GPU backend interface. The render graph layer SHALL depend on the GPU runtime layer for
memory management, state tracking, work graph execution, and feature emulation -- it SHALL NOT depend
directly on the GPU backend interface. No backend-specific types, enums, error codes, or resource
handles SHALL appear in the render graph or GPU runtime API or implementation. The GPU backend SHALL
have no knowledge of render graph or GPU runtime concepts. This prevents leaky abstractions and
ensures that backend changes do not propagate upward.

## R-1.1.8 Excluded Scope

The following SHALL be out of scope:
- Legacy pipelines: geometry shaders, tessellation, traditional vertex pipeline
- Console and mobile platforms: Xbox, PlayStation, Switch, iOS, Android
- CPU-driven scene rendering (CPU-side draw loop for scene geometry)
