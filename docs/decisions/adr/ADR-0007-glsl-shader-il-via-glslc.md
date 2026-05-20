# GLSL as sole shader IL via glslc

## Status

Accepted — 2024-11-20 (backfilled 2026-05-20)

## Context

A Vulkan-only engine ([ADR-0002](ADR-0002-vulkan-sole-gpu-api.md)) consumes SPIR-V on the GPU.
Authors need a shader source language. The choices are:

- **GLSL** — long-standing standard, mature `glslc` compiler from the Vulkan SDK.
- **HLSL via DXC** — Microsoft's choice; targets SPIR-V via `dxc` with the Vulkan SPIR-V
  backend.
- **Slang** — newer multi-target language with strong type system.
- **Custom DSL** — lots of effort for marginal benefit.

DXC's SPIR-V output has historically lagged behind D3D12; HLSL's primary tooling target is
D3D12 not Vulkan. Slang is promising but adds a new dependency plus authors need to learn a
new language.

## Decision

GLSL is the sole shader source language. The build pipeline invokes `glslc` (the Vulkan SDK
compiler) as a subprocess. SPIR-V is produced offline at asset processing time and at hot
reload time. The engine ships no embedded shader compiler library; everything goes through
`glslc`.

Material graphs ([render-styles.md](../../design/rendering/render-styles.md)) and VFX effect
graphs ([effects.md](../../design/rendering/render-effects.md)) codegen GLSL source; that
source goes through the same `glslc` path.

## Consequences

- The engine installer bundles the Vulkan SDK shader tools.
- HLSL is not supported. Authors writing HLSL must port to GLSL (no one-step conversion is
  shipped).
- Shader hot reload re-runs `glslc` per changed shader stage. Sub-second compile is the target.
- No embedded compiler library means failures are limited to the subprocess and the engine
  cannot crash inside a third-party compiler.
- Caching of `glslc` output (SPIR-V plus reflection metadata) is part of the asset pipeline.

## Alternatives Considered

- **HLSL via DXC** — rejected because HLSL's primary target is D3D12 and Vulkan parity has
  historically lagged. Adopting HLSL would either commit the engine to D3D12 alongside Vulkan
  ([ADR-0002](ADR-0002-vulkan-sole-gpu-api.md) rejects this) or accept second-class Vulkan
  support.
- **Slang** — promising long term; potential reconsideration once tooling matures and
  multi-target SPIR-V backends are stable.
- **Custom shader DSL** — high authoring effort for limited benefit; existing GLSL idioms
  cover the engine's use cases.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Shader Pipeline"
- [docs/design/rendering/render-pipeline.md](../../design/rendering/render-pipeline.md)
- [docs/design/rendering/shader-variants.md](../../design/rendering/shader-variants.md)
