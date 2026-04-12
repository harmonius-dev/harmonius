# Asset Pipeline ↔ Rendering Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.2.1.1 | Shader graph to HLSL codegen | PBR material graph | Valid HLSL source, compiles with dxc | IR-5.2.1 |
| TC-IR-5.2.1.2 | Shader graph with custom nodes | Graph with user-defined nodes | HLSL with monomorphized functions | IR-5.2.1 |
| TC-IR-5.2.2.1 | dxc produces valid DXIL | HLSL SM 6.0 source | DXIL blob passes validation | IR-5.2.2 |
| TC-IR-5.2.2.2 | dxc produces valid SPIR-V | HLSL SM 6.0 source | SPIR-V passes spirv-val | IR-5.2.2 |
| TC-IR-5.2.3.1 | MSC produces valid metallib | DXIL from dxc | metallib loads on Metal device | IR-5.2.3 |
| TC-IR-5.2.4.1 | BC7 texture loads to GPU | PNG source, Windows profile | BC7 texture bound in draw call | IR-5.2.4 |
| TC-IR-5.2.4.2 | ASTC texture loads to GPU | PNG source, iOS profile | ASTC texture bound in draw call | IR-5.2.4 |
| TC-IR-5.2.5.1 | Meshlet buffer GPU upload | glTF mesh, 10K triangles | Meshlet buffer with 64v/124t clusters | IR-5.2.5 |
| TC-IR-5.2.5.2 | Meshlet bounds correct | glTF mesh with known AABB | Meshlet bounds within source AABB | IR-5.2.5 |
| TC-IR-5.2.6.1 | Shader hot-reload swaps pipeline | Modified HLSL source | New pipeline active next frame | IR-5.2.6 |
| TC-IR-5.2.6.2 | Shader error keeps old pipeline | Invalid HLSL modification | Previous pipeline still renders | IR-5.2.6 |
| TC-IR-5.2.7.1 | Mip streaming delivers to GPU | 4K texture, request mips 0-3 | Mips resident within 500 ms | IR-5.2.7 |
| TC-IR-5.2.7.2 | LOD streaming delivers meshes | Mesh with 4 LODs, request LOD 0 | LOD 0 resident within budget | IR-5.2.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.2.2.B1 | Cold shader compile (dxc) | < 500 ms | IR-5.2.2 |
| TC-IR-5.2.2.B2 | Cached shader compile | < 1 ms | IR-5.2.2 |
| TC-IR-5.2.4.B1 | BC7 compression 1K texture | 100+ tex/s | IR-5.2.4 |
| TC-IR-5.2.5.B1 | Meshlet build 50K triangles | < 50 ms | IR-5.2.5 |
| TC-IR-5.2.7.B1 | Texture mip stream-in latency | < 500 ms | IR-5.2.7 |
| TC-IR-5.2.7.B2 | Streaming I/O throughput | >= 80% disk BW | IR-5.2.7 |
