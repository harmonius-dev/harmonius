# Asset Processing — Test Cases

Companion to [asset-processing.md](asset-processing.md).

Test case IDs use `TC-12.2.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                | Req       |
|---------------|-------------------------------------|-----------|
| TC-12.2.1.1   | `test_texture_bc7_compress_desktop` | R-12.2.1  |
| TC-12.2.1.2   | `test_texture_astc_apple_silicon`   | R-12.2.1  |
| TC-12.2.1.3   | `test_texture_etc2_mobile_fallback` | R-12.2.1  |
| TC-12.2.1.4   | `test_texture_psnr_threshold`       | R-12.2.1  |
| TC-12.2.2.1   | `test_lod_chain_ratios`             | R-12.2.2  |
| TC-12.2.2.2   | `test_lod_silhouette_error_bound`   | R-12.2.2  |
| TC-12.2.2.3   | `test_lod_count_matches_config`     | R-12.2.2  |
| TC-12.2.3.1   | `test_meshlet_max_vertices`         | R-12.2.3  |
| TC-12.2.3.2   | `test_meshlet_max_triangles`        | R-12.2.3  |
| TC-12.2.3.3   | `test_meshlet_aabb_bounds`          | R-12.2.3  |
| TC-12.2.3.4   | `test_meshlet_normal_cone`          | R-12.2.3  |
| TC-12.2.4.1   | `test_vertex_cache_acmr_improves`   | R-12.2.4  |
| TC-12.2.4.2   | `test_overdraw_ratio_reduced`       | R-12.2.4  |
| TC-12.2.5.1   | `test_lightmap_uv_no_overlap`       | R-12.2.5  |
| TC-12.2.5.2   | `test_lightmap_uv_density_uniform`  | R-12.2.5  |
| TC-12.2.5.3   | `test_lightmap_atlas_packing`       | R-12.2.5  |
| TC-12.2.6.1   | `test_audio_opus_voice_encode`      | R-12.2.6  |
| TC-12.2.6.2   | `test_audio_adpcm_sfx_encode`       | R-12.2.6  |
| TC-12.2.6.3   | `test_audio_pcm_passthrough`        | R-12.2.6  |
| TC-12.2.7.1   | `test_shader_graph_to_hlsl`         | R-12.2.7  |
| TC-12.2.7.2   | `test_shader_graph_no_template_marker` | R-12.2.7 |
| TC-12.2.7.3   | `test_shader_graph_node_to_function` | R-12.2.7 |
| TC-12.2.7.4   | `test_shader_graph_variant_pruning` | R-12.2.7  |
| TC-12.2.8.1   | `test_dep_graph_topological_order`  | R-12.2.8  |
| TC-12.2.8.2   | `test_dep_graph_circular_detect`    | R-12.2.8  |
| TC-12.2.8.3   | `test_dep_graph_impact_analysis`    | R-12.2.8  |
| TC-12.2.9.1   | `test_dxc_dxil_compile`             | R-12.2.9  |
| TC-12.2.9.2   | `test_dxc_spirv_compile`            | R-12.2.9  |
| TC-12.2.9.3   | `test_metal_shaderconverter_msl`    | R-12.2.9  |
| TC-12.2.9.4   | `test_dxc_dead_code_elimination`    | R-12.2.9  |
| TC-12.2.9.5   | `test_dxc_error_line_mapping`       | R-12.2.9  |

1. **TC-12.2.1.1** `test_texture_bc7_compress_desktop` — Compress a 256×256 RGBA8 source with the
   `Desktop` profile. Assert output format is BC7 and block count matches expected.
   - Input: 256×256 RGBA8 buffer, `PlatformProfile::for_target(PlatformTarget::DesktopWindows)`
   - Expected: `ProcessResult.format == TextureFormat::Bc7`, `block_count == 64 * 64`

2. **TC-12.2.1.2** `test_texture_astc_apple_silicon` — Same source, `MacOsAppleSilicon` profile.
   Assert output is ASTC 4×4 with correct block count.
   - Input: 256×256 RGBA8, `PlatformProfile::for_target(PlatformTarget::MacOsAppleSilicon)`
   - Expected: `format == TextureFormat::Astc4x4`, `block_count == 64 * 64`

3. **TC-12.2.1.3** `test_texture_etc2_mobile_fallback` — Mobile profile without ASTC support. Assert
   ETC2 fallback is selected.
   - Input: 256×256 RGBA8, `PlatformProfile::for_target(PlatformTarget::AndroidEtc2Fallback)`
   - Expected: `format == TextureFormat::Etc2Rgba8`

4. **TC-12.2.1.4** `test_texture_psnr_threshold` — Compress test image at quality preset `Standard`.
   Decompress and compute PSNR vs source. Assert PSNR ≥ 40 dB.
   - Input: standard test image, BC7 quality `Standard`
   - Expected: `psnr(source, decompressed) >= 40.0`

5. **TC-12.2.2.1** `test_lod_chain_ratios` — Generate LODs from a 10k-tri mesh with ratios
   `[1.0, 0.5, 0.25, 0.125]`. Assert each LOD's triangle count is within ±5% of target.
   - Input: 10k-tri mesh, `LodConfig { ratios: [1.0, 0.5, 0.25, 0.125] }`
   - Expected: tri counts ≈ [10000, 5000, 2500, 1250], deviation < 5%

6. **TC-12.2.2.2** `test_lod_silhouette_error_bound` — Compute Hausdorff distance between LOD0 and
   LOD3 silhouettes. Assert distance < `error_threshold` from config.
   - Input: 10k-tri sphere, `error_threshold = 0.02 * bounding_radius`
   - Expected: silhouette deviation below threshold

7. **TC-12.2.2.3** `test_lod_count_matches_config` — Request 5 LOD levels. Assert produced chain has
   exactly 5 LODs in order from finest to coarsest.
   - Input: `LodConfig { levels: 5 }`
   - Expected: `lod_chain.len() == 5`, monotonically decreasing tri counts

8. **TC-12.2.3.1** `test_meshlet_max_vertices` — Build meshlets from a 5k-tri mesh. Assert no
   meshlet has more than 64 vertices.
   - Input: 5000-triangle mesh
   - Expected: `meshlets.iter().all(|m| m.vertex_count <= 64)`

9. **TC-12.2.3.2** `test_meshlet_max_triangles` — Same input. Assert no meshlet has more than 124
   triangles.
   - Input: 5000-triangle mesh
   - Expected: `meshlets.iter().all(|m| m.triangle_count <= 124)`

10. **TC-12.2.3.3** `test_meshlet_aabb_bounds` — For each meshlet, verify all referenced vertices
    lie inside the meshlet's AABB.
    - Input: 1000-triangle mesh
    - Expected: every vertex `v` of meshlet `m` satisfies `m.aabb.contains(v)`

11. **TC-12.2.3.4** `test_meshlet_normal_cone` — For each meshlet, verify the dot product of every
    face normal with the cone axis is ≥ `cos(cone_angle)`.
    - Input: meshlet from a sphere segment
    - Expected: `face_normal.dot(cone.axis) >= cone.cos_angle` for all faces

12. **TC-12.2.4.1** `test_vertex_cache_acmr_improves` — Measure ACMR before and after vertex cache
    optimization. Assert post-optimization ACMR is lower.
    - Input: unoptimized indexed mesh, 5000 triangles
    - Expected: `acmr_after < acmr_before`, `acmr_after < 1.5`

13. **TC-12.2.4.2** `test_overdraw_ratio_reduced` — Measure overdraw ratio before and after overdraw
    optimization on a sphere mesh. Assert ratio decreases.
    - Input: 1000-triangle sphere, frontal projection
    - Expected: `overdraw_after < overdraw_before`

14. **TC-12.2.5.1** `test_lightmap_uv_no_overlap` — Generate lightmap UVs for a cube. Assert no
    chart UV polygons overlap (rasterize to 256×256 mask).
    - Input: 12-triangle cube mesh
    - Expected: `count_overlapping_pixels(uvs) == 0`

15. **TC-12.2.5.2** `test_lightmap_uv_density_uniform` — Compute texel density per chart for a cube.
    Assert variance < 5% of mean.
    - Input: 12-triangle cube mesh, `target_texel_density = 32`
    - Expected: `density_variance / density_mean < 0.05`

16. **TC-12.2.5.3** `test_lightmap_atlas_packing` — Pack 50 charts into a 1024×1024 atlas. Assert
    all charts fit, no overlap, packing efficiency > 60%.
    - Input: 50 charts of varied sizes
    - Expected: all fit in `1024×1024`, no overlap, `packed_area / total_area > 0.60`

17. **TC-12.2.6.1** `test_audio_opus_voice_encode` — Encode 1 s of 48 kHz mono speech with the
    `Voice` preset. Assert output is Opus with bitrate ≈ 24 kbps.
    - Input: 48000-sample mono speech buffer, `AudioPreset::Voice`
    - Expected: `format == AudioEncoding::Opus`, `bitrate ∈ [20_000, 28_000]`

18. **TC-12.2.6.2** `test_audio_adpcm_sfx_encode` — Encode a short 22 kHz SFX buffer with the
    `ShortSfx` preset. Assert output is ADPCM and roughly half the size of source.
    - Input: 4096-sample 16-bit PCM buffer
    - Expected: `format == AudioEncoding::Adpcm`, `output.len() ≈ source.len() / 4`

19. **TC-12.2.6.3** `test_audio_pcm_passthrough` — Encode a latency-critical clip with the
    `LatencyCritical` preset. Assert output is raw PCM and bytes equal source.
    - Input: 1024-sample 16-bit PCM buffer
    - Expected: `format == AudioEncoding::Pcm16`, `output_bytes == source_bytes`

20. **TC-12.2.7.1** `test_shader_graph_to_hlsl` — Compile a 3-node graph (texture sample → tint →
    output). Assert generated HLSL contains a `main()` entry point and a `Texture2D` declaration.
    - Input: graph `{ TextureSample("Albedo") → Multiply([1,0,0,1]) → Output }`
    - Expected: HLSL string contains `Texture2D Albedo`, contains `float4 main`, compiles via dxc

21. **TC-12.2.7.2** `test_shader_graph_no_template_marker` — Generated HLSL must not contain `{{`,
    `}}`, `<%`, `%>`, or any other template marker.
    - Input: any non-trivial shader graph
    - Expected: regex `r"(\{\{|\}\}|<%|%>)"` finds no match in output

22. **TC-12.2.7.3** `test_shader_graph_node_to_function` — Each graph node emits exactly one HLSL
    function. For a 5-node graph, assert 5 helper functions plus one entry point.
    - Input: 5-node logic graph
    - Expected: HLSL contains 5 distinct `<type> nodeN_<name>(...)` functions

23. **TC-12.2.7.4** `test_shader_graph_variant_pruning` — Graph has 3 boolean static switches but
    only 4 of 8 combinations are reachable. Assert only 4 variants are emitted.
    - Input: 3 static switches, reachability info from graph analysis
    - Expected: `generated_variants.len() == 4`

24. **TC-12.2.8.1** `test_dep_graph_topological_order` — Build a DAG with edges `A→B, B→C, A→C`.
    Assert `topological_order()` returns `[C, B, A]` (leaves first).
    - Input: 3-node DAG with described edges
    - Expected: result is `[C, B, A]` or any valid topological order with leaves first

25. **TC-12.2.8.2** `test_dep_graph_circular_detect` — Add edges `A→B, B→A`. Assert
    `topological_order()` returns `Err(GraphError::Cycle { path })`.
    - Input: 2-node graph with mutual edges
    - Expected: `Err(GraphError::Cycle { path: vec![A, B, A] })`

26. **TC-12.2.8.3** `test_dep_graph_impact_analysis` — DAG `Tex → Mat1, Tex → Mat2, Mat1 → Scn`.
    Assert `transitive_dependents(Tex) == {Mat1, Mat2, Scn}`.
    - Input: 4-node DAG
    - Expected: returned set equals `{Mat1, Mat2, Scn}`

27. **TC-12.2.9.1** `test_dxc_dxil_compile` — Run dxc CLI on a trivial pixel shader. Assert output
    is DXIL bytecode beginning with magic `BC\xC0\xDE`.
    - Input: `float4 main() : SV_Target { return float4(1,0,0,1); }`, target `ps_6_0`
    - Expected: dxc subprocess exit 0, output starts with DXIL magic bytes

28. **TC-12.2.9.2** `test_dxc_spirv_compile` — Same shader, target SPIR-V. Assert output starts with
    magic `0x07230203`.
    - Input: same shader, dxc flag `-spirv`
    - Expected: first u32 of output == `0x07230203`

29. **TC-12.2.9.3** `test_metal_shaderconverter_msl` — Run metal-shaderconverter on a DXIL blob.
    Assert output is a valid metallib (header magic `MTLB`).
    - Input: DXIL bytes from TC-12.2.9.1
    - Expected: subprocess exit 0, output bytes contain `b"MTLB"` near start

30. **TC-12.2.9.4** `test_dxc_dead_code_elimination` — Compile a shader with an unreachable branch.
    Reflect the bytecode; assert the dead branch's instructions are absent.
    - Input: `if (false) { discard; } return float4(1,0,0,1);`
    - Expected: reflected bytecode has no `discard` instruction

31. **TC-12.2.9.5** `test_dxc_error_line_mapping` — Compile a shader generated from a graph with a
    deliberate type error at HLSL line 42 (graph node `mul_node`). Assert error report contains line
    42 and the originating graph node name.
    - Input: HLSL with type mismatch, source map mapping line 42 to `mul_node`
    - Expected: `CompilationError { hlsl_line: 42, graph_node: Some("mul_node") }`

## Integration Tests

| ID           | Name                                | Req       |
|--------------|-------------------------------------|-----------|
| TC-12.2.I.1  | `test_full_processing_graph`        | R-12.2.8  |
| TC-12.2.I.2  | `test_incremental_only_changed`     | R-12.2.8  |
| TC-12.2.I.3  | `test_shader_graph_full_pipeline`   | R-12.2.7  |
| TC-12.2.I.4  | `test_texture_processor_per_target` | R-12.2.1  |
| TC-12.2.I.5  | `test_mesh_lod_meshlet_chain`       | R-12.2.2  |
| TC-12.2.I.6  | `test_audio_multi_preset_batch`     | R-12.2.6  |
| TC-12.2.I.7  | `test_processing_parallel_fanout`   | R-12.2.8  |

1. **TC-12.2.I.1** `test_full_processing_graph` — Build a processing graph from 50 mixed assets.
   Assert all 50 nodes process in topological order, no node runs before its dependencies, and final
   report contains 50 success entries.
   - Input: 50 assets with random dep edges (validated as DAG)
   - Expected: `Report { processed: 50, failed: 0 }`, no out-of-order execution observed

2. **TC-12.2.I.2** `test_incremental_only_changed` — Process 100 assets fresh (cache cold). Modify 1
   source. Re-process. Assert only 1 processor invocation occurs and unaffected assets are skipped
   via the incremental cache.
   - Input: 100 independent assets, modify asset `42` source
   - Expected: second pass invokes processor 1 time; 99 cache hits

3. **TC-12.2.I.3** `test_shader_graph_full_pipeline` — Compile a 20-node shader graph end-to-end:
   graph → HLSL → DXIL → MSL. Assert the final metallib loads on Metal and the HLSL roundtrips
   through dxc with no warnings.
   - Input: 20-node PBR shader graph
   - Expected: all 3 outputs present in CAS, HLSL passes IDE-compatible parse, no template markers,
     dxc warnings == 0

4. **TC-12.2.I.4** `test_texture_processor_per_target` — Process a single 1K texture with three
   `PlatformProfile` targets. Assert three distinct `ArtifactKey`s are produced.
   - Input: 1024×1024 RGBA8 source, profiles desktop, Apple Silicon, mobile
   - Expected: 3 different artifact keys, each format matches profile

5. **TC-12.2.I.5** `test_mesh_lod_meshlet_chain` — End-to-end mesh pipeline: source → LOD chain →
   meshlet build → vertex cache opt. Assert each LOD has meshlets respecting 64v/124t and improved
   ACMR.
   - Input: 50k-tri mesh, 4 LOD levels
   - Expected: 4 LODs, each with valid meshlets and ACMR < 1.5

6. **TC-12.2.I.6** `test_audio_multi_preset_batch` — Encode 30 audio clips: 10 voice (Opus), 10 SFX
   (ADPCM), 10 latency-critical (PCM). Assert correct format per preset.
   - Input: 30 mixed-preset audio clips
   - Expected: 10 Opus, 10 ADPCM, 10 PCM artifacts in CAS

7. **TC-12.2.I.7** `test_processing_parallel_fanout` — Process 100 independent textures on a thread
   pool with 8 workers. Assert all 8 workers stay busy and total wall time is at least 4× faster
   than single-threaded.
   - Input: 100 textures, 8-thread pool
   - Expected: `wall_time_parallel * 4 < wall_time_serial`

## Benchmarks

| ID            | Benchmark                          | Target          | Req       |
|---------------|------------------------------------|-----------------|-----------|
| TC-12.2.B.1   | BC7 compress 1024×1024             | < 10 ms         | R-12.2.1  |
| TC-12.2.B.2   | LOD chain (10k tri, 4 levels)      | < 200 ms        | R-12.2.2  |
| TC-12.2.B.3   | Meshlet build (50k tri)            | < 100 ms        | R-12.2.3  |
| TC-12.2.B.4   | Shader graph compile (cold)        | < 500 ms        | R-12.2.9  |
| TC-12.2.B.5   | Shader cache hit                   | < 1 ms          | R-12.2.9  |
| TC-12.2.B.6   | Incremental rebuild (1 changed)    | < 2 s           | R-12.2.8  |

1. **TC-12.2.B.1** — Compress one 1024×1024 RGBA8 source to BC7 with the `Standard` quality preset.
   Wall time end-to-end. Measured with `criterion`.

2. **TC-12.2.B.2** — Generate 4 LOD levels from a 10k-triangle mesh using edge-collapse
   simplification. Wall time from `process()` invocation to all LODs written.

3. **TC-12.2.B.3** — Partition a 50k-triangle LOD0 mesh into meshlets (64v/124t bounds) and compute
   per-meshlet AABB and normal cones. Wall time end-to-end.

4. **TC-12.2.B.4** — Cold compile of a 20-node shader graph: graph → HLSL → DXC subprocess → DXIL.
   Wall time from `process()` to artifact stored in CAS. Excludes Metal stage.

5. **TC-12.2.B.5** — Re-process the same shader graph with cache populated. Wall time should be
   bounded by cache lookup + metadata load only.

6. **TC-12.2.B.6** — Process a 100-asset dependency graph after modifying exactly one source. Wall
   time from `process_all` call to final report. Validates incremental rebuild path.
