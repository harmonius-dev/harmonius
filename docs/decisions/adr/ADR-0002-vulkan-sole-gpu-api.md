# Vulkan 1.4 as sole GPU API

## Status

Accepted — 2024-09-04 (backfilled 2026-05-20)

## Context

The engine targets Windows, macOS, Linux, iOS, Android, and consoles. Historically, supporting this
matrix required juggling D3D12 (Windows), Metal (Apple), and Vulkan (Linux/Android). Each backend
carries different pipeline state objects, descriptor models, barrier idioms, and shader ILs.
Maintaining three backends doubles the surface area of every GPU feature.

Vulkan 1.4 ratified mesh shaders, ray tracing, work graphs (via vendor extensions), and
hardware-accelerated path-tracing primitives. macOS Vulkan support via `MoltenVK` was the historical
wedge but the project intentionally targets native Vulkan on Apple via `ash` plus the runtime
translation layer that ships with newer macOS.

## Decision

Vulkan 1.4 is the sole GPU API on every platform. Rust accesses Vulkan via the `ash` crate. There is
no D3D12 backend, no Metal backend, no DXC, no HLSL, and no MoltenVK as a runtime dependency. Shader
IL is GLSL compiled offline to SPIR-V via the `glslc` subprocess
([ADR-0007](ADR-0007-glsl-shader-il-via-glslc.md)).

## Consequences

- One pipeline model, one descriptor model, one barrier model across all platforms.
- Mesh shaders and ray tracing are minimum requirements; older devices are not supported.
- macOS support requires a Vulkan-on-Metal translation layer where native Vulkan is not
  available. The engine assumes Vulkan-on-Metal exists at the deployment substrate.
- Pipeline state objects, descriptor pools, and timeline semaphores are the canonical
  primitives; no D3D-style descriptor heaps or fences.
- All GPU runtime requirements live under [R-2.14](../../requirements/rendering/gpu-runtime.md).

## Alternatives Considered

- **WebGPU as universal API** — promising but immature; the engine targets desktop and console
  AAA, not browsers.
- **Triple backend (D3D12 + Metal + Vulkan)** — rejected because the maintenance cost
  triples for every renderer feature (PSOs, barriers, descriptor models, RT pipelines).
- **D3D12 only on Windows, Vulkan elsewhere** — Windows-specific D3D12 path bifurcates the
  rendering team and the GPU runtime.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Language and Toolchain", "Backend
  Language Matrix"
- [docs/design/rendering/render-pipeline.md](../../design/rendering/render-pipeline.md)
- [docs/requirements/rendering/gpu-runtime.md](../../requirements/rendering/gpu-runtime.md)
