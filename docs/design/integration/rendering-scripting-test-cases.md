# Rendering ↔ Scripting Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.5.1.1 | PBR material graph emits HLSL | Graph: albedo texture + metallic slider | Valid HLSL with SurfaceOutput struct | IR-3.5.1 |
| TC-IR-3.5.1.2 | Unlit material graph emits HLSL | Graph: color constant only | HLSL with Unlit shading model | IR-3.5.1 |
| TC-IR-3.5.1.3 | Normal map node emits tangent code | Graph with normal map sample | HLSL includes TBN transform | IR-3.5.1 |
| TC-IR-3.5.2.1 | dxc produces DXIL from HLSL | Valid PBR pixel shader HLSL | DXIL binary, zero errors | IR-3.5.2 |
| TC-IR-3.5.2.2 | dxc produces SPIR-V from HLSL | Valid PBR pixel shader HLSL | SPIR-V binary, zero errors | IR-3.5.2 |
| TC-IR-3.5.2.3 | metal-shaderconverter produces MSL | Valid DXIL binary | metallib binary, zero errors | IR-3.5.2 |
| TC-IR-3.5.3.1 | Permutation key generates variants | DefaultLit + AlphaTest + ForwardPlus | Unique PSO per combination | IR-3.5.3 |
| TC-IR-3.5.3.2 | Feature ifdef blocks correct | Graph with optional emissive | `#ifdef HAS_EMISSIVE` present | IR-3.5.3 |
| TC-IR-3.5.4.1 | Hot reload swaps shader | Modify albedo color in graph | New shader active next frame | IR-3.5.4 |
| TC-IR-3.5.4.2 | Hot reload preserves state | Change roughness param | Other material params unchanged | IR-3.5.4 |
| TC-IR-3.5.5.1 | PP graph emits compute HLSL | Bloom threshold + blur graph | Valid compute shader HLSL | IR-3.5.5 |
| TC-IR-3.5.5.2 | PP pass registers in render graph | Compiled PP compute shader | Pass node in graph with I/O | IR-3.5.5 |
| TC-IR-3.5.6.1 | Effect graph emits particle HLSL | Spawn + gravity + color-over-life | Three compute kernels in HLSL | IR-3.5.6 |
| TC-IR-3.5.6.2 | Effect kernels dispatch correctly | Compiled particle kernels | GPU dispatch with correct args | IR-3.5.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.5.1.B1 | HLSL codegen 100-node graph | < 50 ms | IR-3.5.1 |
| TC-IR-3.5.2.B1 | dxc compile single shader | < 500 ms | IR-3.5.2 |
| TC-IR-3.5.3.B1 | 16 permutation compile batch | < 5 s total | IR-3.5.3 |
| TC-IR-3.5.4.B1 | Hot reload turnaround | < 1 s (R-13.4.NF2) | IR-3.5.4 |
