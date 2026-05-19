# Rendering ↔ Scripting Integration Test Cases

All test cases are CI-runnable unless marked `[GPU]`, which require the GPU test runners. Negative
tests are marked `[NEG]`. Unit tests run without any device. Integration tests may mmap fixture
files. Benchmarks run on the perf CI lane.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.5.1.U1 | `codegen_glsl` is pure | Same graph, 1000 runs | Byte-identical GLSL output | IR-3.5.1 |
| TC-IR-3.5.1.U2 | Arena GLSL handle roundtrip | Emit + resolve handle | GLSL bytes match source | IR-3.5.1 |
| TC-IR-3.5.1.U3 | PBR node to GLSL sort | 3-node PBR graph | Topological order emitted | IR-3.5.1 |
| TC-IR-3.5.3.U1 | `PermutationKey` hash stable | Same fields, 1000 hashes | Identical u64 result | IR-3.5.3 |
| TC-IR-3.5.3.U2 | Feature bit layout | All 9 flags set | `ShaderFeatures(0x1FF)` | IR-3.5.3 |
| TC-IR-3.5.3.U3 | Permutation sorted Vec lookup | 64 variants | Binary search returns index | IR-3.5.3 |
| TC-IR-3.5.4.U1 | `PipelineStateHandle` generation | Publish, free, re-publish | Old handle resolves `None` | IR-3.5.4 |
| TC-IR-3.5.4.U2 | Channel-polled not async | Worker drain loop | No tokio / futures symbols | IR-3.5.4 |
| TC-IR-3.5.U1 | rkyv align `MaterialShaderOutput` | Serialize + mmap | ptr % 16 == 0 | IR-3.5.1 |
| TC-IR-3.5.U2 | rkyv align `CompiledEffect` | Serialize + mmap | ptr % 16 == 0 | IR-3.5.6 |
| TC-IR-3.5.U3 | rkyv `check_bytes` accepts valid | Fresh archive | Validation passes | IR-3.5.1 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.5.1.I1 | PBR graph emits GLSL | Albedo tex + metallic slider | Valid GLSL w/ `SurfaceOutput` | IR-3.5.1 |
| TC-IR-3.5.1.I2 | Unlit graph emits GLSL | Color constant only | GLSL with `Unlit` model | IR-3.5.1 |
| TC-IR-3.5.1.I3 | Normal map node emits TBN | Graph w/ normal sample | GLSL includes TBN transform | IR-3.5.1 |
| TC-IR-3.5.2.I1 | glslc produces SPIR-V | Valid PBR pixel GLSL | SPIR-V passes validation | IR-3.5.2 |
| TC-IR-3.5.2.I2 | glslc produces SPIR-V | Valid PBR pixel GLSL | SPIR-V passes `spirv-val` | IR-3.5.2 |
| TC-IR-3.5.2.I3 | glslc produces SPIR-V module | SPIR-V from glslc | SPIR-V module loads on Vulkan `[GPU]` | IR-3.5.2 |
| TC-IR-3.5.2.I4 | Main-thread subprocess owner | Compile request | Worker never touches pipe | IR-3.5.2 |
| TC-IR-3.5.3.I1 | Permutation enumeration | DefaultLit+AlphaClip+FwdPlus | Unique PSO per combo | IR-3.5.3 |
| TC-IR-3.5.3.I2 | Feature ifdef blocks | Graph w/ optional emissive | `#ifdef HAS_EMISSIVE` present | IR-3.5.3 |
| TC-IR-3.5.3.I3 | Parallel permutation compile | 16 variants | All compiled within budget | IR-3.5.3 |
| TC-IR-3.5.4.I1 | Hot reload swaps pipeline | Modify albedo in graph | New PSO active next frame `[GPU]` | IR-3.5.4 |
| TC-IR-3.5.4.I2 | Hot reload preserves state | Change roughness param | Other params unchanged | IR-3.5.4 |
| TC-IR-3.5.4.I3 | MPSC back-pressure | 200 reqs, cap 64 | Producer blocks; none dropped | IR-3.5.4 |
| TC-IR-3.5.4.I4 | Render-thread PSO publish | Worker -> render channel | PSO created on render thread | IR-3.5.4 |
| TC-IR-3.5.4.I5 | `ShaderReloadStatus` flow | Success + failure | ECS resource updated on both | IR-3.5.4 |
| TC-IR-3.5.5.I1 | PP graph emits compute GLSL | Bloom threshold + blur | Valid compute GLSL | IR-3.5.5 |
| TC-IR-3.5.5.I2 | PP pass registers in graph | Compiled PP compute | Pass node in render graph | IR-3.5.5 |
| TC-IR-3.5.6.I1 | Effect graph emits kernels | Spawn+gravity+color | 4 compute kernels in GLSL | IR-3.5.6 |
| TC-IR-3.5.6.I2 | Effect kernels dispatch | Compiled kernels | GPU dispatch w/ correct args `[GPU]` | IR-3.5.6 |
| TC-IR-3.5.6.I3 | `CompiledEffect` rkyv load | mmap fixture | Zero-copy kernel access | IR-3.5.6 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.5.1.N1 | `[NEG]` GLSL codegen error | Cyclic graph | Error node; prior handle kept | IR-3.5.1 |
| TC-IR-3.5.1.N2 | `[NEG]` Unknown node kind | Graph w/ unresolved node | Build fails with descriptive error | IR-3.5.1 |
| TC-IR-3.5.2.N1 | `[NEG]` glslc compile error | Syntactically invalid GLSL | Exit != 0; stderr captured | IR-3.5.2 |
| TC-IR-3.5.2.N2 | `[NEG]` glslc missing binary | CLI not installed | Graceful error; build fails | IR-3.5.2 |
| TC-IR-3.5.2.N3 | `[NEG]` glslc translation error | Malformed SPIR-V | Previous SPIR-V module retained | IR-3.5.2 |
| TC-IR-3.5.3.N1 | `[NEG]` Missing permutation | Unknown `PermutationKey` | Falls back to default permutation | IR-3.5.3 |
| TC-IR-3.5.4.N1 | `[NEG]` Hot reload compile fail | Invalid GLSL edit | Previous pipeline still renders | IR-3.5.4 |
| TC-IR-3.5.4.N2 | `[NEG]` Stale handle resolve | Freed + reused slot | `resolve` returns `None` | IR-3.5.4 |
| TC-IR-3.5.4.N3 | `[NEG]` glslc stderr to overlay | Compile error, 3 diagnostics | `Failed{3}` in ECS resource | IR-3.5.4 |
| TC-IR-3.5.5.N1 | `[NEG]` PP graph invalid binding | Missing input texture | Build fails; pass not registered | IR-3.5.5 |
| TC-IR-3.5.6.N1 | `[NEG]` Effect kernel compile fail | Invalid spawn GLSL | Effect disabled; warning emitted | IR-3.5.6 |
| TC-IR-3.5.U.N1 | `[NEG]` rkyv misalignment | Misaligned fixture | `check_bytes` rejects | IR-3.5.1 |
| TC-IR-3.5.U.N2 | `[NEG]` Corrupt `CompiledEffect` | Tampered bytes | Load fails; red placeholder | IR-3.5.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.5.1.B1 | GLSL codegen 100-node graph | < 50 ms | IR-3.5.1 |
| TC-IR-3.5.1.B2 | `MaterialShaderCache` arena insert | < 5 us | IR-3.5.1 |
| TC-IR-3.5.2.B1 | Cold `glslc` compile single shader | < 500 ms | IR-3.5.2 |
| TC-IR-3.5.2.B2 | Cached `glslc` compile | < 1 ms | IR-3.5.2 |
| TC-IR-3.5.3.B1 | 16 permutation compile batch | < 5 s total | IR-3.5.3 |
| TC-IR-3.5.3.B2 | Permutation sorted-Vec lookup (256) | < 200 ns | IR-3.5.3 |
| TC-IR-3.5.4.B1 | Hot reload end-to-end | < 1 s (R-13.4.NF2) | IR-3.5.4 |
| TC-IR-3.5.4.B2 | `PipelineStateTable::publish` | < 10 us | IR-3.5.4 |
| TC-IR-3.5.4.B3 | `PipelineStateTable::resolve` | < 100 ns | IR-3.5.4 |
| TC-IR-3.5.6.B1 | `CompiledEffect` rkyv load | Zero heap allocations | IR-3.5.6 |

## Coverage Summary

| IR | Unit | Integration | Negative | Benchmark |
|----|------|-------------|----------|-----------|
| IR-3.5.1 | 3 | 3 | 2 | 2 |
| IR-3.5.2 | 0 | 4 | 3 | 2 |
| IR-3.5.3 | 3 | 3 | 1 | 2 |
| IR-3.5.4 | 2 | 5 | 3 | 3 |
| IR-3.5.5 | 0 | 2 | 1 | 0 |
| IR-3.5.6 | 0 | 3 | 1 | 1 |

All six integration requirements have at least one integration test, at least one negative test
where a runtime failure mode exists, and at least one benchmark where a performance budget applies.
Cross-cutting rkyv / alignment tests sit under `TC-IR-3.5.U*`.

## CI Lanes

| Lane | Test kinds | Runner |
|------|-----------|--------|
| `unit` | Unit | Any Rust toolchain, no device |
| `integration-cpu` | Integration w/o `[GPU]`, all negative | Any Rust toolchain |
| `integration-gpu` | `[GPU]`-marked | GPU test runners (Win, macOS, Linux) |
| `bench` | Benchmark | Dedicated perf machines |

`[NEG]`-marked tests run on `integration-cpu` and form part of the default PR gate. GPU tests are
promoted to the PR gate on changes to rendering or scripting crates.
