# HLSL Coding Standard

## Scope

Shader intermediate language. All shaders are authored in HLSL and compiled via DXC to DXIL (D3D12)
and SPIR-V (Vulkan). Metal shaders use Metal Shader Converter (DXIL to MSL).

## Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Struct | `PascalCase` | `VertexOutput` |
| Function | `PascalCase` | `ComputeLighting` |
| Variable (local) | `camelCase` | `worldPos` |
| Constant buffer | `PascalCase` + `CB` suffix | `FrameDataCB` |
| Texture | `PascalCase` | `AlbedoMap` |
| Sampler | `camelCase` + `Sampler` suffix | `linearSampler` |
| Macro | `SCREAMING_SNAKE` | `MAX_LIGHTS` |
| Semantic | `SCREAMING_SNAKE` | `SV_Position` |

## File Organization

- `.hlsl` extension for shader source
- `.hlsli` extension for include files
- One entry point per file (VS, PS, CS, etc.)
- Shared utilities in `common/` include directory
- Group by render pass or feature

## Formatting Rules

- 100-character line limit
- 4-space indentation
- Braces on same line
- One declaration per line

## Linting Rules

- DXC compiler warnings treated as errors
- No implicit truncation warnings
- All resources must have explicit binding

## Type Checking

- DXC compilation with target shader model
- Validate resource binding completeness
- Check for uninitialized variables

## How to Check and Fix

| Validation | Check command | Fix command |
|------------|--------------|-------------|
| Compile | `dxc -T {target} {file}` | (manual) |

## Project-Specific Rules

1. **HLSL only** — no GLSL, no MSL authoring
2. **DXC pipeline** — all compilation through DXC
3. **Shader model 6.6+** — minimum for mesh shaders and ray tracing
4. **Bindless resources** — use descriptor indexing for resource access
5. **Wave intrinsics** — prefer wave ops over shared memory where applicable

## Cache-Friendly Patterns

- **Coalesced memory access** — structure buffer reads for cache-line alignment
- **SoA for compute** — struct-of-arrays layout for GPU compute shaders
- **Minimize register pressure** — keep live variables low for occupancy
- **Shared memory tiling** — load tiles into group shared memory for reuse
- **Avoid divergent branches** — keep warps/wavefronts coherent

## Testing

1. **Compilation tests** — every shader must compile with DXC for all target APIs
2. **Visual regression** — screenshot comparison for rendering correctness
3. **GPU validation** — run with API validation layers enabled
4. **Performance benchmarks** — measure frame time and GPU occupancy

## Best Practices

1. Use `cbuffer` for per-frame constants, `StructuredBuffer` for per-object
2. Prefer `half` precision where full `float` is not needed
3. Use `[numthreads(64, 1, 1)]` as default compute group size
4. Document all entry points with purpose and inputs
5. Use `#include` for shared code; avoid duplication
6. Minimize texture fetches in pixel shaders
7. Use `SV_DispatchThreadID` and friends consistently
8. Keep shader permutations manageable with `#ifdef`
9. Profile with GPU profiler before optimizing
10. Use `static const` for shader-local constants

## Anti-Patterns

1. **GLSL or MSL authoring** — HLSL is the sole source
2. **Unbounded loops** — GPU stalls; use fixed bounds
3. **Dynamic branching in pixel shaders** — causes quad divergence
4. **Large constant buffers** — split by update frequency (per-frame, per-object, per-material)
5. **Unstructured `#ifdef` soup** — use feature permutation system
6. **Global textures** — use bindless descriptor indexing
7. **Redundant barriers** — let the render graph manage transitions
8. **Magic numbers** — use named constants
9. **Shared memory bank conflicts** — pad to avoid conflicts
10. **Ignoring precision** — use `half` where `float` is unnecessary
