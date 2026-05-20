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
| TC-12.2.9.3   | `test_naga_spirv`    | R-12.2.9  |
| TC-12.2.9.4   | `test_dxc_dead_code_elimination`    | R-12.2.9  |
| TC-12.2.9.5   | `test_dxc_error_line_mapping`       | R-12.2.9  |
| TC-12.5.1.1   | `test_vfs_unified_namespace`        | R-12.5.1  |
| TC-12.5.2.1   | `test_async_read_no_std_fs`         | R-12.5.2  |
| TC-12.5.3.1   | `test_gpu_direct_storage_submit`    | R-12.5.3  |
| TC-12.5.4.1   | `test_texture_residency_request`    | R-12.5.4  |
| TC-12.5.5.1   | `test_mesh_residency_coarse_fine`   | R-12.5.5  |
| TC-12.5.6.1   | `test_priority_queue_order_and_age` | R-12.5.6  |
| TC-12.5.7.1   | `test_budget_monitor_pressure`      | R-12.5.7  |
| TC-12.5.8.1   | `test_pak_archive_o1_lookup`        | R-12.5.8  |
| TC-12.5.9.1   | `test_chunk_codec_selection`        | R-12.5.9  |
| TC-12.5.10.1  | `test_cdn_download_hash_verify`     | R-12.5.10 |

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
    output). Assert generated GLSL contains a `main()` entry point and a `Texture2D` declaration.
    - Input: graph `{ TextureSample("Albedo") → Multiply([1,0,0,1]) → Output }`
    - Expected: GLSL string contains `Texture2D Albedo`, contains `float4 main`, compiles via naga

21. **TC-12.2.7.2** `test_shader_graph_no_template_marker` — Generated GLSL must not contain `{{`,
    `}}`, `<%`, `%>`, or any other template marker.
    - Input: any non-trivial shader graph
    - Expected: regex `r"(\{\{|\}\}|<%|%>)"` finds no match in output

22. **TC-12.2.7.3** `test_shader_graph_node_to_function` — Each graph node emits exactly one GLSL
    function. For a 5-node graph, assert 5 helper functions plus one entry point.
    - Input: 5-node logic graph
    - Expected: GLSL contains 5 distinct `<type> nodeN_<name>(...)` functions

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

27. **TC-12.2.9.1** `test_dxc_dxil_compile` — Run naga on a trivial pixel shader. Assert output
    is SPIR-V bytecode beginning with magic `BC\xC0\xDE`.
    - Input: `float4 main() : SV_Target { return float4(1,0,0,1); }`, target `ps_6_0`
    - Expected: naga in-process compilation exit 0, output starts with SPIR-V magic bytes

28. **TC-12.2.9.2** `test_dxc_spirv_compile` — Same shader, target SPIR-V. Assert output starts with
    magic `0x07230203`.
    - Input: same shader, naga flag `-spirv`
    - Expected: first u32 of output == `0x07230203`

29. **TC-12.2.9.3** `test_naga_spirv` — Compile GLSL to SPIR-V via naga. Assert output starts with
    SPIR-V magic `0x07230203`.
    - Input: SPIR-V bytes from TC-12.2.9.1
    - Expected: valid SPIR-V header `0x07230203` near start

30. **TC-12.2.9.4** `test_dxc_dead_code_elimination` — Compile a shader with an unreachable branch.
    Reflect the bytecode; assert the dead branch's instructions are absent.
    - Input: `if (false) { discard; } return float4(1,0,0,1);`
    - Expected: reflected bytecode has no `discard` instruction

31. **TC-12.2.9.5** `test_dxc_error_line_mapping` — Compile a shader generated from a graph with a
    deliberate type error at GLSL line 42 (graph node `mul_node`). Assert error report contains line
    42 and the originating graph node name.
    - Input: GLSL with type mismatch, source map mapping line 42 to `mul_node`
    - Expected: `CompilationError { glsl_line: 42, graph_node: Some("mul_node") }`

32. **TC-12.5.1.1** `test_vfs_unified_namespace` — Mount the same logical path under a loose-file
    store, a pak archive, and a mock HTTP store. Read bytes from `"textures/brick.ktx2"` through
    each mount and assert identical content.
    - Input: three `BackingStore` variants mounted at `/content`, one asset present in each
    - Expected: `resolve()` returns a `ResolvedAsset` from every mount, identical content bytes,
      `unmount` removes paths from `list()`

33. **TC-12.5.2.1** `test_async_read_no_std_fs` — Read a 100 MB asset through the streaming runtime
    and assert zero `std::fs` calls occur in the stream path via static inspection of linked
    symbols.
    - Input: 100 MB asset, `StreamingManager::load(..., StreamPriority::Normal, ...)`
    - Expected: completion via tokio task; no `std::fs::*` symbol referenced in the
      `harmonius_content::streaming` crate

34. **TC-12.5.3.1** `test_gpu_direct_storage_submit` — On a backend reporting
    `StagingBufferBackend::is_available() == true`, submit a `GpuTransferRequest` for a 16 MB
    compressed texture. Assert the destination GPU buffer contains the decompressed pixels.
    - Input: 16 MB Zstd-compressed texture, valid `GpuBufferHandle`, `codec: Zstd`
    - Expected: `submit().await == Ok(())`, sampled GPU pixels match reference decode

35. **TC-12.5.4.1** `test_texture_residency_request` — Request mips 0..3 for one texture then assert
    `is_resident` for each mip. Evict above mip 1 and assert only mips 0..1 remain resident.
    - Input: `TextureResidency::new(64 * 1024 * 1024)`, 1 texture, mips 0..3
    - Expected: all four mips resident after request, exactly two mips resident after
      `evict_mips(id, above_mip = 1)`

36. **TC-12.5.5.1** `test_mesh_residency_coarse_fine` — Request LOD 0 (fine) for a 2 MB mesh while
    LOD 3 (coarse) is already resident. Assert both LODs are resident after the async load
    completes.
    - Input: `MeshResidency::new(32 * 1024 * 1024)`, mesh with 4 LODs, LOD 3 preloaded
    - Expected: `is_resident(id, 0)` true, `is_resident(id, 3)` true, no eviction of coarse LOD

37. **TC-12.5.6.1** `test_priority_queue_order_and_age` — Enqueue 100 `Prefetch` requests then 10
    `Critical` requests, then tick 500 frames. Assert critical requests dequeue first and that one
    aged prefetch is promoted above prefetch baseline.
    - Input: `PriorityQueue` with `aging_interval_frames = 60`, mixed-priority batch
    - Expected: first 10 dequeues are `Critical`; after aging, at least one prefetch is ordered
      above newer prefetch entries

38. **TC-12.5.7.1** `test_budget_monitor_pressure` — Allocate against a 512 MB GPU budget until 95%
    used. Assert `pressure_level()` reports `Critical` and a further `allocate` returns `false`.
    - Input: `BudgetConfig { gpu_budget_bytes: 512 * 1024 * 1024, ... }`
    - Expected: `pressure_level()` transitions `None → Warning → Critical`, `allocate` returns
      `false` at saturation

39. **TC-12.5.8.1** `test_pak_archive_o1_lookup` — Build a pak with 10,000 entries. Look up a random
    asset ID and assert the returned `PakEntry` matches the inserted offset/size, and that lookup
    cost is independent of entry count (hash-based directory).
    - Input: 10,000 pak entries, random query across full ID range
    - Expected: lookup returns correct `PakEntry`, asymptotic cost `O(1)` verified by timing
      1e3/1e4/1e5 archives within 2× variance

40. **TC-12.5.9.1** `test_chunk_codec_selection` — Write a pak containing one audio chunk (LZ4) and
    one texture chunk (Zstd). Read each chunk and assert the codec flag matches the expected
    `CompressionCodec` variant, and that decompression roundtrips byte-for-byte.
    - Input: audio chunk, texture chunk
    - Expected: `entry.codec == Lz4` for audio, `entry.codec == Zstd` for texture, decompressed
      bytes equal originals

41. **TC-12.5.10.1** `test_cdn_download_hash_verify` — Resolve a VFS path whose store is a mock CDN
    with a corrupted chunk. Assert download is rejected with `IntegrityError` and retried, then
    assert a second fetch with correct bytes succeeds and caches locally.
    - Input: mock CDN returning corrupted bytes on first fetch, correct bytes on second
    - Expected: first fetch returns `Err(IntegrityError { expected, actual })`, second fetch
      succeeds, third access is a local cache hit

## Integration Tests

| ID           | Name                                | Req       |
|--------------|-------------------------------------|-----------|
| TC-12.2.I.1  | `test_full_processing_graph`        | R-12.2.8   |
| TC-12.2.I.2  | `test_incremental_only_changed`     | R-12.2.8   |
| TC-12.2.I.3  | `test_shader_graph_full_pipeline`   | R-12.2.7   |
| TC-12.2.I.4  | `test_texture_processor_per_target` | R-12.2.1   |
| TC-12.2.I.5  | `test_mesh_lod_meshlet_chain`       | R-12.2.2   |
| TC-12.2.I.6  | `test_audio_multi_preset_batch`     | R-12.2.6   |
| TC-12.2.I.7  | `test_processing_parallel_fanout`   | R-12.2.8   |
| TC-12.2.I.8  | `us_artist_texture_preset_import`   | US-12.2.1  |
| TC-12.2.I.9  | `us_build_eng_platform_overrides`   | US-12.2.2  |
| TC-12.2.I.10 | `us_env_artist_auto_lod_chain`      | US-12.2.3  |
| TC-12.2.I.11 | `us_artist_lod_ratio_config`        | US-12.2.4  |
| TC-12.2.I.12 | `us_eng_meshlet_bounds_for_culling` | US-12.2.5  |
| TC-12.2.I.13 | `us_artist_vertex_order_optimized`  | US-12.2.6  |
| TC-12.2.I.14 | `us_env_artist_auto_lightmap_uvs`   | US-12.2.7  |
| TC-12.2.I.15 | `us_audio_designer_preset_encoding` | US-12.2.8  |
| TC-12.2.I.16 | `us_artist_audio_preset_params`     | US-12.2.9  |
| TC-12.2.I.17 | `us_artist_readable_hlsl_output`    | US-12.2.12 |
| TC-12.2.I.18 | `us_build_eng_dep_graph_incremental`| US-12.2.13 |
| TC-12.2.I.19 | `us_artist_circular_dep_reported`   | US-12.2.14 |
| TC-12.2.I.20 | `us_artist_shader_error_click_thru` | US-12.2.16 |
| TC-12.2.I.21 | `us_designer_auto_collider_import`  | US-12.2.18 |
| TC-12.2.I.22 | `us_artist_collider_algo_choice`    | US-12.2.19 |
| TC-12.2.I.23 | `us_env_artist_collider_coresident` | US-12.2.20 |
| TC-12.2.I.24 | `us_eng_shader_reflection_all_backs`| US-12.2.15 |
| TC-12.5.I.1  | `us_eng_vfs_unified_namespace`      | US-12.5.1  |
| TC-12.5.I.2  | `us_build_eng_vfs_path_decoupling`  | US-12.5.2  |
| TC-12.5.I.3  | `us_eng_async_direct_io_reads`      | US-12.5.3  |
| TC-12.5.I.4  | `us_eng_file_to_gpu_dma`            | US-12.5.4  |
| TC-12.5.I.5  | `us_env_artist_mip_screen_density`  | US-12.5.5  |
| TC-12.5.I.6  | `us_env_artist_mesh_lod_crossfade`  | US-12.5.6  |
| TC-12.5.I.7  | `us_artist_coarse_lod_always_resid` | US-12.5.7  |
| TC-12.5.I.8  | `us_eng_priority_queue_scheduling`  | US-12.5.8  |
| TC-12.5.I.9  | `us_eng_coalesce_and_age_requests`  | US-12.5.9  |
| TC-12.5.I.10 | `us_artist_budget_progressive_evict`| US-12.5.10 |
| TC-12.5.I.11 | `us_gamer_cdn_on_first_access`      | US-12.5.15 |
| TC-12.5.I.12 | `us_eng_gpu_direct_storage_stream`  | US-12.5.16 |
| TC-12.5.I.13 | `us_eng_gpu_compute_decompression`  | US-12.5.17 |
| TC-12.5.I.14 | `us_dev_residency_lru_prefetch`     | US-12.5.18 |
| TC-12.5.I.15 | `us_gamer_evict_on_os_pressure`     | US-12.5.19 |

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
   graph → GLSL → SPIR-V → SPIR-V. Assert the final SPIR-V module loads on Vulkan and the GLSL
   roundtrips through naga with no warnings.
   - Input: 20-node PBR shader graph
   - Expected: all 3 outputs present in CAS, GLSL passes IDE-compatible parse, no template markers,
     naga warnings == 0

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

8. **TC-12.2.I.8** `us_artist_texture_preset_import` — As a technical artist, import a folder of 10
   RGBA8 textures through the `Desktop` preset and 10 through the `MacOsAppleSilicon` preset without
   touching config. Assert desktop artifacts are BC7 and Apple artifacts are ASTC 4×4.
   - Input: 20 RGBA8 source textures, two preset selections via import dialog
   - Expected: 10 BC7 + 10 ASTC 4×4 artifacts in CAS, zero manual format overrides

9. **TC-12.2.I.9** `us_build_eng_platform_overrides` — As a build engineer, run a batch build with a
   per-platform override table that selects BC7 for `Desktop`, ASTC for Apple Silicon, and ETC2 for
   `AndroidEtc2Fallback`. Assert every target produces the expected format with no manual
   intervention.
   - Input: override table `{desktop: Bc7, apple: Astc4x4, android: Etc2Rgba8}`, 30 textures
   - Expected: 90 artifacts generated (30 × 3), each matching the override table

10. **TC-12.2.I.10** `us_env_artist_auto_lod_chain` — As an environment artist, import a 20k-tri
    mesh with default settings (no LOD authoring). Assert a 4-level LOD chain is produced
    automatically and each LOD is addressable in the processed mesh.
    - Input: 20k-tri mesh, default `LodConfig`
    - Expected: `lod_chain.len() == 4`, each LOD present in CAS, monotonically decreasing tri count

11. **TC-12.2.I.11** `us_artist_lod_ratio_config` — As a technical artist, set per-level ratios
    `[1.0, 0.6, 0.3, 0.1]` and error thresholds in the import settings. Assert the produced chain
    matches the configured ratios within ±5% and respects each error bound.
    - Input: 15k-tri mesh, custom `LodConfig`
    - Expected: tri counts ≈ [15000, 9000, 4500, 1500], Hausdorff error ≤ configured threshold

12. **TC-12.2.I.12** `us_eng_meshlet_bounds_for_culling` — As an engine developer, process a LOD
    mesh and run the GPU culling pipeline against the generated meshlet AABB + normal cone. Assert
    back-facing meshlets are culled and visible ones rendered.
    - Input: 10k-tri sphere LOD, camera inside sphere facing +Z
    - Expected: meshlets with `cone.axis.dot(view_dir) < -cone.cos_angle` culled, front meshlets
      drawn

13. **TC-12.2.I.13** `us_artist_vertex_order_optimized` — As a technical artist, process a mesh and
    assert both vertex cache ACMR and overdraw ratio improve vs the raw imported order.
    - Input: 8k-tri mesh, unoptimized indices
    - Expected: ACMR drops below 1.5, overdraw ratio decreases by ≥ 20%

14. **TC-12.2.I.14** `us_env_artist_auto_lightmap_uvs` — As an environment artist, process a static
    building mesh with no lightmap UV authoring. Assert a non-overlapping UV atlas is generated at
    uniform texel density.
    - Input: 200-tri building mesh, `target_texel_density = 32`
    - Expected: zero UV overlap (rasterized 1024² mask), density variance < 5%

15. **TC-12.2.I.15** `us_audio_designer_preset_encoding` — As an audio designer, import 3 clips
    tagged `Voice`, `ShortSfx`, and `LatencyCritical`. Assert the artifacts use Opus, ADPCM, and
    PCM16 respectively.
    - Input: 3 WAV clips, 3 preset tags
    - Expected: `[Opus, Adpcm, Pcm16]` matching tags

16. **TC-12.2.I.16** `us_artist_audio_preset_params` — As a technical artist, create two import
    presets with different Opus bitrates (32 kbps vs 96 kbps) and process the same voice clip
    through each. Assert the resulting artifact bitrates differ as configured.
    - Input: 5 s voice clip, two Opus presets
    - Expected: first artifact bitrate ∈ [28k, 36k], second ∈ [88k, 104k]

17. **TC-12.2.I.17** `us_artist_readable_hlsl_output` — As a technical artist, compile a 10-node PBR
    graph and open the generated GLSL in an IDE parser. Assert the parser reports zero errors and
    each node emits a comment containing the graph node ID.
    - Input: 10-node PBR graph
    - Expected: GLSL has one `// node: nodeN` comment per graph node, IDE parser returns 0 errors

18. **TC-12.2.I.18** `us_build_eng_dep_graph_incremental` — As a build engineer, process 500 assets
    with a dep graph (texture → material → mesh → scene). Touch one texture; re-run processing.
    Assert only dependents of that texture are reprocessed.
    - Input: 500-asset DAG, modify `textures/tex_42.png`
    - Expected: reprocess count equals transitive dependents of tex_42; all others cache hit

19. **TC-12.2.I.19** `us_artist_circular_dep_reported` — As a technical artist, intentionally create
    a circular reference `Mat_A → Mat_B → Mat_A` via import metadata. Assert processing halts with a
    `CycleDetected` error naming both materials.
    - Input: 2 materials with mutual references
    - Expected: `Err(ProcessingError::CycleDetected { cycle: [Mat_A, Mat_B, Mat_A] })`

20. **TC-12.2.I.20** `us_artist_shader_error_click_thru` — As a technical artist, edit a graph node
    to produce a type error. Assert the error report contains both the GLSL line number and the
    originating graph node identifier so the editor can jump to the node.
    - Input: 8-node graph with invalid `mul` types
    - Expected: `CompilationError { glsl_line: N, graph_node: Some("mul_node") }` where N > 0

21. **TC-12.2.I.21** `us_designer_auto_collider_import` — As a game designer, import a prop mesh
    with default settings and assert a collision shape is produced and stored in the processed
    artifact without manual authoring.
    - Input: 500-tri prop mesh, default import settings
    - Expected: processed artifact contains `CollisionShape`, physics backend loads it

22. **TC-12.2.I.22** `us_artist_collider_algo_choice` — As a technical artist, select V-HACD for one
    mesh, quickhull for a second, and AABB for a third via import settings. Assert each produces the
    chosen shape variant.
    - Input: 3 meshes with 3 collider choices
    - Expected: artifacts contain `CollisionShape::ConvexDecomp`, `::ConvexHull`, `::Aabb`
      respectively

23. **TC-12.2.I.23** `us_env_artist_collider_coresident` — As an environment artist, load a
    processed mesh at runtime. Assert the physics binding attaches the collision shape without
    requiring a separate asset reference.
    - Input: processed mesh artifact containing collider
    - Expected: single `load(mesh_id)` yields both render mesh and collider; physics world contains
      the body

24. **TC-12.2.I.24** `us_eng_shader_reflection_all_backs` — As an engine developer, compile one GLSL
    source through the pipeline to SPIR-V. Assert reflection data extracted from each
    backend reports identical binding layouts and push-constant ranges.
    - Input: single PBR GLSL source, naga → SPIR-V, naga → SPIR-V, naga → SPIR-V
    - Expected: all three reflections contain the same descriptor-set/binding tuples and
      push-constant ranges; no mismatches

## Integration Tests — Streaming (US-12.5)

1. **TC-12.5.I.1** `us_eng_vfs_unified_namespace` — As an engine developer, mount loose files, a
   pak, and a mock HTTP store and fetch an asset by VFS path. Assert every subsystem (loader,
   streaming, editor browser) reaches the asset through the same `VfsPath`.
   - Input: 3 backing stores with one asset each, 3 subsystem clients
   - Expected: every client returns identical content bytes via `vfs.resolve(path)`

2. **TC-12.5.I.2** `us_build_eng_vfs_path_decoupling` — As a build engineer, switch a project
   between a loose-file development layout and a shipping pak layout without changing any asset path
   strings. Assert all in-engine references still resolve.
   - Input: two layouts with identical VFS paths, sample project referencing 50 assets
   - Expected: 50/50 resolve in both layouts; zero path rewrites required

3. **TC-12.5.I.3** `us_eng_async_direct_io_reads` — As an engine developer, load a 100 MB pak entry
   through the streaming manager while running two worker threads doing CPU-bound work. Assert
   neither worker is blocked on I/O and throughput reaches ≥ 80% of raw sequential disk BW.
   - Input: 100 MB entry, 2 busy workers, NVMe device
   - Expected: workers make forward progress during load; measured BW ≥ 80% of raw

4. **TC-12.5.I.4** `us_eng_file_to_gpu_dma` — As an engine developer, stream a 256 MB compressed
   texture pack directly into GPU memory through `StagingBufferBackend`. Assert CPU utilization
   stays below 5% during the transfer.
   - Input: 256 MB Zstd pack, staging-capable backend
   - Expected: transfer completes, CPU busy < 5%, final pixels match reference

5. **TC-12.5.I.5** `us_env_artist_mip_screen_density` — As an environment artist, place 1,000
   textures in a scene. Assert the residency manager holds only mips matching each texture's current
   screen-space density.
   - Input: 1,000 textures, camera at fixed pose
   - Expected: for each texture, resident mip range equals `mip_for_density(screen_texels)` ± 1

6. **TC-12.5.I.6** `us_env_artist_mesh_lod_crossfade` — As an environment artist, walk a camera
   across a 10,000-mesh scene. Capture 60 frames during LOD transitions. Assert dithered cross-fade
   is applied and no popping is visible (delta > threshold per pixel).
   - Input: 10,000 meshes with 4 LODs each, camera path
   - Expected: transition frames contain dither pattern; frame-to-frame color delta < threshold

7. **TC-12.5.I.7** `us_artist_coarse_lod_always_resid` — As a technical artist, mark coarsest LOD as
   permanently resident. Scroll the camera 5 km across terrain. Assert coarse LOD never evicts.
   - Input: mesh LOD chain with resident-coarse flag, 5 km camera sweep
   - Expected: `is_resident(id, coarsest_lod) == true` at every frame

8. **TC-12.5.I.8** `us_eng_priority_queue_scheduling` — As an engine developer, submit 100
   low-priority prefetch requests then 10 frame-critical requests mid-stream. Assert all
   frame-critical requests complete before any pending prefetch.
   - Input: 100 `Prefetch` + 10 `Critical` requests
   - Expected: all 10 critical requests finish before the next prefetch dequeues

9. **TC-12.5.I.9** `us_eng_coalesce_and_age_requests` — As an engine developer, submit 50 adjacent
   chunk requests (contiguous file offsets) and 10 stale prefetches older than the aging interval.
   Assert the scheduler issues a single coalesced I/O and the stale prefetches are promoted.
   - Input: 50 adjacent requests, 10 stale prefetches, `aging_interval_frames = 60`
   - Expected: coalesced issue count ≤ 5 (20%+ reduction), stale prefetches advance priority tier

10. **TC-12.5.I.10** `us_artist_budget_progressive_evict` — As a technical artist, set a 512 MB GPU
    budget and load assets until pressure hits `Emergency`. Assert progressive eviction occurs and
    the process does not OOM.
    - Input: `gpu_budget_bytes = 512 MB`, asset stream exceeding budget
    - Expected: `evict_to_budget` runs, post-eviction usage ≤ 512 MB, no crash, no OOM

11. **TC-12.5.I.11** `us_gamer_cdn_on_first_access` — As a game player, launch a game with zero
    local assets cached. Open a level referencing 20 assets via the CDN manifest. Assert each asset
    downloads on first access, passes hash verification, and is cached for subsequent accesses.
    - Input: empty local cache, 20-asset CDN manifest
    - Expected: 20 downloads on first frame, 0 downloads on second visit, hash match on all

12. **TC-12.5.I.12** `us_eng_gpu_direct_storage_stream` — As an engine developer, stream 500
    textures into GPU memory via the platform direct-storage path (Vulkan staging buffers, Vulkan
    staging buffers, or io_uring staging). Assert NVMe bandwidth saturates (≥ 3 GB/s) and CPU memory
    bandwidth is not consumed by copies.
    - Input: 500 textures (total 2 GB), platform DS backend
    - Expected: sustained ≥ 3 GB/s, CPU memcpy traffic ≈ 0 (via perf counter)

13. **TC-12.5.I.13** `us_eng_gpu_compute_decompression` — As an engine developer, load compressed
    streamed assets and decompress them in place on the GPU via compute. Assert CPU usage stays low
    during the decompression phase.
    - Input: 200 Zstd-compressed chunks, compute decompress pipeline
    - Expected: CPU busy < 5% while GPU decompresses, decompressed bytes match reference

14. **TC-12.5.I.14** `us_dev_residency_lru_prefetch` — As a game developer, configure per-asset type
    budgets and enable camera-driven prefetch. Walk a camera across an open-world path. Assert LRU
    eviction keeps within budgets and prefetch hides latency (< 1% visible pop-ins).
    - Input: texture/mesh budgets, 2 km camera path
    - Expected: budgets never exceeded, pop-in events < 1% of frames

15. **TC-12.5.I.15** `us_gamer_evict_on_os_pressure` — As a game player, run a long session
    triggering an OS memory pressure signal. Assert the engine evicts unused assets on signal and
    the session continues without OOM.
    - Input: long-running session, synthetic `MemoryPressureLevel::Emergency` signal
    - Expected: eviction frees ≥ 20% of managed memory, session continues, no crash

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

4. **TC-12.2.B.4** — Cold compile of a 20-node shader graph: graph → GLSL → naga in-process
   compilation → SPIR-V. Wall time from `process()` to artifact stored in CAS. Excludes naga compile
   stage.

5. **TC-12.2.B.5** — Re-process the same shader graph with cache populated. Wall time should be
   bounded by cache lookup + metadata load only.

6. **TC-12.2.B.6** — Process a 100-asset dependency graph after modifying exactly one source. Wall
   time from `process_all` call to final report. Validates incremental rebuild path.
