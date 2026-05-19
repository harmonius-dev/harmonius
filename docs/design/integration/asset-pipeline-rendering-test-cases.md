# Asset Pipeline ↔ Rendering Integration Test Cases

All test cases are CI-runnable unless marked `[GPU]`, which require the GPU test runners. Negative
tests are marked `[NEG]`. Unit tests run without any device. Integration tests may mmap fixture
files. Benchmarks run on the perf CI lane.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.2.1.U1 | GLSL codegen sort | 3-node PBR graph | Topological order emitted | IR-5.2.1 |
| TC-IR-5.2.1.U2 | Codegen is pure | Same graph, 1000 runs | Byte-identical GLSL output | IR-5.2.1 |
| TC-IR-5.2.4.U1 | Texture profile select | Win + PNG RGBA8 | `Bc7Unorm` chosen | IR-5.2.4 |
| TC-IR-5.2.4.U2 | Texture profile select | iOS + PNG RGBA8 | `Astc4x4Unorm` chosen | IR-5.2.4 |
| TC-IR-5.2.4.U3 | Texture profile select | macOS + PNG RGBA8 | `Astc4x4Unorm` chosen | IR-5.2.4 |
| TC-IR-5.2.5.U1 | Meshlet cluster limit | 1024 verts, 1024 tris | ceil(1024/64) clusters, 124-tri cap | IR-5.2.5 |
| TC-IR-5.2.5.U2 | Normal cone compute | Flat quad | Cone half-angle == 0 | IR-5.2.5 |
| TC-IR-5.2.6.U1 | `PipelineStateHandle` generation | Publish, free, re-publish | Old handle resolves `None` | IR-5.2.6 |
| TC-IR-5.2.6.U2 | Variant lookup sorted Vec | 100 variants | Binary search returns correct index | IR-5.2.6 |
| TC-IR-5.2.7.U1 | `StreamRequestTable` submit | Priority 5 | Handle with generation 0 | IR-5.2.7 |
| TC-IR-5.2.7.U2 | `StreamRequestTable::poll` | Mark ready, poll | State == `Ready` | IR-5.2.7 |
| TC-IR-5.2.U1 | rkyv align check `BakedTexture` | Serialize + mmap | `data` ptr % 16 == 0 | IR-5.2.4 |
| TC-IR-5.2.U2 | rkyv align check `CompiledShader` | Serialize + mmap | `bytecode` ptr % 16 == 0 | IR-5.2.2 |
| TC-IR-5.2.U3 | `ArchivedVec` zero-copy | mmap fixture, read `data` | No heap allocation (allocator probe) | IR-5.2.4 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.2.1.I1 | Shader graph to GLSL codegen | PBR material graph | Valid GLSL source, glslc accepts | IR-5.2.1 |
| TC-IR-5.2.1.I2 | Shader graph custom nodes | User-defined nodes | GLSL with monomorphized functions | IR-5.2.1 |
| TC-IR-5.2.2.I1 | glslc produces valid SPIR-V | GLSL SM 6.0 | SPIR-V passes validation | IR-5.2.2 |
| TC-IR-5.2.2.I2 | glslc produces valid SPIR-V | GLSL SM 6.0 | SPIR-V passes spirv-val | IR-5.2.2 |
| TC-IR-5.2.2.I3 | Main-thread subprocess ownership | Compile request | Worker never touches pipe | IR-5.2.2 |
| TC-IR-5.2.3.I1 | glslc produces valid SPIR-V module | SPIR-V from glslc | SPIR-V module loads on Vulkan device `[GPU]` | IR-5.2.3 |
| TC-IR-5.2.4.I1 | BC7 texture loads to GPU | PNG, Windows profile | BC7 bound in draw call `[GPU]` | IR-5.2.4 |
| TC-IR-5.2.4.I2 | ASTC texture loads on iOS | PNG, iOS profile | ASTC bound `[GPU]` | IR-5.2.4 |
| TC-IR-5.2.4.I3 | ASTC texture loads on macOS | PNG, macOS profile | Same ASTC blob as iOS | IR-5.2.4 |
| TC-IR-5.2.5.I1 | Meshlet buffer GPU upload | glTF, 10K tris | 64v/124t clusters `[GPU]` | IR-5.2.5 |
| TC-IR-5.2.5.I2 | Meshlet bounds correct | glTF with known AABB | Cluster bounds within source AABB | IR-5.2.5 |
| TC-IR-5.2.5.I3 | Android meshlet emulation | glTF on Vulkan w/o ext | Indirect draw path renders | IR-5.2.5 |
| TC-IR-5.2.6.I1 | Shader hot-reload swap | Modified GLSL | New pipeline next frame `[GPU]` | IR-5.2.6 |
| TC-IR-5.2.6.I2 | MPSC channel back-pressure | 200 compile reqs, cap 64 | Producer blocks, none dropped | IR-5.2.6 |
| TC-IR-5.2.6.I3 | `ShaderReloadUiEvent` emit | Success + failure path | Overlay receives both events | IR-5.2.6 |
| TC-IR-5.2.7.I1 | Mip streaming delivers | 4K texture, mips 0-3 | Mips resident in <= 500 ms `[GPU]` | IR-5.2.7 |
| TC-IR-5.2.7.I2 | LOD streaming delivers | Mesh, 4 LODs, req LOD 0 | LOD 0 resident in budget `[GPU]` | IR-5.2.7 |
| TC-IR-5.2.7.I3 | Phase-8 poll ordering | Multiple pending reqs | All completed reqs drained at frame end | IR-5.2.7 |
| TC-IR-5.2.7.I4 | ECS request/handle flow | `load::<Mesh>` call | Handle returns immediately; resolves later | IR-5.2.7 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.2.2.N1 | `[NEG]` glslc compile error | Syntactically invalid GLSL | Exit != 0; stderr captured; old PSO kept | IR-5.2.2 |
| TC-IR-5.2.2.N2 | `[NEG]` glslc missing binary | CLI not installed | Graceful error; asset build fails cleanly | IR-5.2.2 |
| TC-IR-5.2.3.N1 | `[NEG]` glslc translation error | Malformed SPIR-V | Error path; previous SPIR-V module retained | IR-5.2.3 |
| TC-IR-5.2.4.N1 | `[NEG]` Unsupported format | 10-bit HDR on ETC2 target | Falls back to `Rgba8UnormFallback` | IR-5.2.4 |
| TC-IR-5.2.4.N2 | `[NEG]` Corrupt PNG | Truncated file | Load error; red placeholder shown | IR-5.2.4 |
| TC-IR-5.2.5.N1 | `[NEG]` Meshlet build fails | Degenerate triangles | Logged; excluded from draw list | IR-5.2.5 |
| TC-IR-5.2.6.N1 | `[NEG]` Shader error keeps old | Invalid GLSL edit | Previous pipeline still renders | IR-5.2.6 |
| TC-IR-5.2.6.N2 | `[NEG]` Stale handle resolve | Freed + reused slot | `resolve` returns `None` via generation | IR-5.2.6 |
| TC-IR-5.2.6.N3 | `[NEG]` glslc stderr to overlay | Compile error, 3 diagnostics | `ShaderReloadStatus::Failed{3}` in ECS resource | IR-5.2.6 |
| TC-IR-5.2.7.N1 | `[NEG]` Streaming I/O error | Truncated asset file | Retries 3x; falls back to lowest resident mip | IR-5.2.7 |
| TC-IR-5.2.7.N2 | `[NEG]` Streaming timeout | Slow disk fixture | Marked `Failed` after budget; render continues | IR-5.2.7 |
| TC-IR-5.2.U.N1 | `[NEG]` mmap alignment fail | Misaligned fixture | rkyv `check_bytes` rejects; asset load fails | IR-5.2.4 |
| TC-IR-5.2.U.N2 | `[NEG]` CAS cache corruption | Tampered entry | Hash mismatch; recompile triggered | IR-5.2.2 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.2.2.B1 | Cold shader compile (glslc) | < 500 ms | IR-5.2.2 |
| TC-IR-5.2.2.B2 | Cached shader compile | < 1 ms | IR-5.2.2 |
| TC-IR-5.2.3.B1 | glslc SPIR-V -> SPIR-V module | < 200 ms | IR-5.2.3 |
| TC-IR-5.2.4.B1 | BC7 compression, 1K texture | 100+ tex/s | IR-5.2.4 |
| TC-IR-5.2.4.B2 | ASTC 4x4 compression, 1K texture | 80+ tex/s | IR-5.2.4 |
| TC-IR-5.2.5.B1 | Meshlet build, 50K triangles | < 50 ms | IR-5.2.5 |
| TC-IR-5.2.6.B1 | Hot-reload end-to-end latency | < 750 ms | IR-5.2.6 |
| TC-IR-5.2.6.B2 | `PipelineStateTable::publish` | < 10 us | IR-5.2.6 |
| TC-IR-5.2.6.B3 | `PipelineStateTable::resolve` | < 100 ns | IR-5.2.6 |
| TC-IR-5.2.7.B1 | Texture mip stream-in latency | < 500 ms | IR-5.2.7 |
| TC-IR-5.2.7.B2 | Streaming I/O throughput | >= 80% disk BW | IR-5.2.7 |
| TC-IR-5.2.7.B3 | `StreamRequestTable::poll` | < 50 ns | IR-5.2.7 |
| TC-IR-5.2.U.B1 | rkyv mmap access, `BakedTexture` | Zero heap allocations | IR-5.2.4 |
| TC-IR-5.2.U.B2 | Variant lookup, 256 variants | < 200 ns (binary search) | IR-5.2.6 |

## Coverage Summary

| IR | Unit | Integration | Negative | Benchmark |
|----|------|-------------|----------|-----------|
| IR-5.2.1 | 2 | 2 | 0 | 0 |
| IR-5.2.2 | 1 | 3 | 2 | 2 |
| IR-5.2.3 | 0 | 1 | 1 | 1 |
| IR-5.2.4 | 4 | 3 | 2 | 2 |
| IR-5.2.5 | 2 | 3 | 1 | 1 |
| IR-5.2.6 | 2 | 3 | 3 | 3 |
| IR-5.2.7 | 2 | 4 | 2 | 3 |

All seven integration requirements have at least one unit or integration test, at least one negative
test where applicable (IR-5.2.1 codegen is pure and has no runtime failure mode), and at least one
benchmark where a performance budget exists.

## CI Lanes

| Lane | Test kinds | Runner |
|------|-----------|--------|
| `unit` | Unit | Any Rust toolchain, no device |
| `integration-cpu` | Integration w/o `[GPU]`, all negative | Any Rust toolchain |
| `integration-gpu` | `[GPU]`-marked | GPU test runners (Win, macOS, Linux) |
| `bench` | Benchmark | Dedicated perf machines |

The `[NEG]`-marked tests run on `integration-cpu` and form part of the default PR gate. GPU tests
are promoted to the PR gate on changes to rendering or asset processing crates.
