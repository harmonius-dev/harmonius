# Asset Pipeline — Test Cases

Companion to [asset-pipeline.md](asset-pipeline.md).

Test case IDs use `TC-12.{group}.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                              | Req       |
|---------------|-----------------------------------|-----------|
| TC-12.1.1.1   | `test_native_format_magic_valid`  | R-12.1.1  |
| TC-12.1.1.2   | `test_native_format_hash_mismatch` | R-12.1.1 |
| TC-12.1.1.3   | `test_native_format_dedup`        | R-12.1.1  |
| TC-12.1.2.1   | `test_png_srgb_decode`            | R-12.1.2  |
| TC-12.1.2.2   | `test_exr_linear_decode`          | R-12.1.2  |
| TC-12.1.3.1   | `test_wav_metadata_extract`       | R-12.1.3  |
| TC-12.1.3.2   | `test_flac_loop_points`           | R-12.1.3  |
| TC-12.1.4.1   | `test_validation_error_offset`    | R-12.1.4  |
| TC-12.1.4.2   | `test_validation_warning_optional` | R-12.1.4 |
| TC-12.1.5.1   | `test_batch_progress_reporting`   | R-12.1.5  |
| TC-12.1.5.2   | `test_batch_cancel_rollback`      | R-12.1.5  |
| TC-12.3.1.1   | `test_cas_store_dedup`            | R-12.3.1  |
| TC-12.3.1.2   | `test_cas_load_by_hash`           | R-12.3.1  |
| TC-12.3.2.1   | `test_metadata_put_get`           | R-12.3.2  |
| TC-12.3.3.1   | `test_cache_hit_skips_processing` | R-12.3.3  |
| TC-12.3.3.2   | `test_cache_miss_on_settings_change` | R-12.3.3 |
| TC-12.3.4.1   | `test_dep_invalidation_transitive` | R-12.3.4 |
| TC-12.3.5.1   | `test_search_full_text`           | R-12.3.5  |
| TC-12.3.5.2   | `test_search_facet_filter`        | R-12.3.5  |
| TC-12.3.6.1   | `test_thumbnail_async_generated`  | R-12.3.6  |
| TC-12.3.10.1  | `test_version_history_restore`    | R-12.3.10 |
| TC-12.4.1.1   | `test_file_watch_debounce`        | R-12.4.1  |
| TC-12.4.1.2   | `test_file_watch_dedupe_rapid`    | R-12.4.1  |
| TC-12.4.2.1   | `test_atomic_pointer_swap`        | R-12.4.2  |
| TC-12.4.2.2   | `test_swap_dependents_updated`    | R-12.4.2  |
| TC-12.4.3.1   | `test_shader_pso_swap`            | R-12.4.3  |
| TC-12.4.3.2   | `test_shader_error_overlay`       | R-12.4.3  |
| TC-12.4.4.1   | `test_logic_graph_state_preserve` | R-12.4.4  |
| TC-12.4.4.2   | `test_logic_graph_layout_change`  | R-12.4.4  |
| TC-12.4.5.1   | `test_ui_subtree_rebuild`         | R-12.4.5  |
| TC-12.4.6.1   | `test_partial_subasset_reimport`  | R-12.4.6  |
| TC-12.4.7.1   | `test_editor_runtime_sync`        | R-12.4.7  |
| TC-12.7.1.1   | `test_asset_header_roundtrip`     | R-12.7.1  |
| TC-12.7.1.2   | `test_asset_section_o1_lookup`    | R-12.7.1  |
| TC-12.7.2.1   | `test_bundle_partial_decompress`  | R-12.7.2  |
| TC-12.7.3.1   | `test_structural_diff_mesh`       | R-12.7.3  |
| TC-12.7.4.1   | `test_three_way_merge_disjoint`   | R-12.7.4  |
| TC-12.7.5.1   | `test_auto_resolve_lww`           | R-12.7.5  |
| TC-12.7.5.2   | `test_auto_resolve_union`         | R-12.7.5  |

1. **TC-12.1.1.1** `test_native_format_magic_valid` — Import a native binary file with magic
   `b"HAST"` and version 1. Assert importer accepts and registers the asset.
   - Input: `bytes[0..4] == *b"HAST"`, `bytes[4..8] == 1u32_le`, valid hash
   - Expected: `import_file` returns `Ok(ImportResult { asset_id, .. })`, metadata stored

2. **TC-12.1.1.2** `test_native_format_hash_mismatch` — Header hash differs from BLAKE3 of body.
   Assert importer rejects with `IntegrityError`.
   - Input: header `content_hash = [0u8; 32]`, body BLAKE3 = real hash
   - Expected: `Err(ImportError::Integrity { expected, actual })`

3. **TC-12.1.1.3** `test_native_format_dedup` — Import the same file twice. Assert second import
   returns `StoreResult::Deduplicated` and CAS contains exactly one blob.
   - Input: identical 4 KiB asset bytes imported twice
   - Expected: first call `Written`, second `Deduplicated`, `cas.exists(hash) == true`, 1 file in
     CAS root

4. **TC-12.1.2.1** `test_png_srgb_decode` — Decode a 4×4 sRGB PNG with red pixels (255,0,0). Assert
   linear values match `srgb_to_linear(1.0)` and color space is tagged sRGB.
   - Input: 4×4 PNG with all-red pixels
   - Expected: `decoded.color_space == ColorSpace::Srgb`, `decoded.pixels[0] ≈ [1.0, 0.0, 0.0, 1.0]`
     after linearization

5. **TC-12.1.2.2** `test_exr_linear_decode` — Decode an EXR with HDR pixel value `(2.5, 1.0, 0.0)`.
   Assert decoded floats match the source and color space is tagged Linear.
   - Input: 1×1 OpenEXR with `R=2.5, G=1.0, B=0.0`
   - Expected: `decoded.color_space == ColorSpace::Linear`,
     `decoded.pixels[0] == [2.5, 1.0, 0.0, 1.0]`

6. **TC-12.1.3.1** `test_wav_metadata_extract` — Decode a 48 kHz / 16-bit / stereo WAV file with a
   `cue` chunk at sample 1024. Assert extracted metadata matches.
   - Input: 1 s WAV with `fmt`, `data`, `cue` chunks
   - Expected: `meta.sample_rate == 48000`, `meta.channels == 2`, `meta.bit_depth == 16`,
     `meta.cue_markers == [1024]`

7. **TC-12.1.3.2** `test_flac_loop_points` — Decode a FLAC with `LOOPSTART=2048` and `LOOPEND=4096`
   Vorbis comments. Assert loop points are extracted.
   - Input: FLAC with Vorbis comment block
   - Expected: `meta.loop_start == Some(2048)`, `meta.loop_end == Some(4096)`

8. **TC-12.1.4.1** `test_validation_error_offset` — Asset with `format_version = 99` (unsupported).
   Assert error includes path, byte offset of the version field, and a fix suggestion.
   - Input: header bytes at offset 4 = `99u32_le`, file path `"asset.har"`
   - Expected: `ValidationError { path: "asset.har", offset: 4, suggestion: Some(_) }`

9. **TC-12.1.4.2** `test_validation_warning_optional` — Asset missing optional `tags` field. Assert
   `ValidationWarning` is emitted, import succeeds.
   - Input: well-formed asset with `tags` omitted
   - Expected: `import_file` returns `Ok`, warnings contain
     `ValidationWarning::MissingOptional { field: "tags" }`

10. **TC-12.1.5.1** `test_batch_progress_reporting` — Batch import 10 files. Assert
    `ProgressTracker.completed` advances from 0 to 10 with one event per file.
    - Input: `Vec<ImportEntry>` of length 10
    - Expected: `progress.poll()` yields 10 `ItemCompleted` events; final `completed == 10`

11. **TC-12.1.5.2** `test_batch_cancel_rollback` — Batch of 100 files; cancel after 50 completed.
    Assert no metadata entries for the cancelled half and CAS gc removes orphan blobs.
    - Input: 100-file batch, `BatchImportHandle::cancel()` after 50
    - Expected: `metadata.query(all).len() == 50`, remaining 50 leave no metadata,
      `cas.gc(referenced).orphans_removed >= 50`

12. **TC-12.3.1.1** `test_cas_store_dedup` — Store the same 1 KiB blob twice. Assert second call
    returns `Deduplicated` and disk usage unchanged.
    - Input: bytes `b"x".repeat(1024)`, hash `h`
    - Expected: first `Ok(Written)`, second `Ok(Deduplicated)`, file count == 1

13. **TC-12.3.1.2** `test_cas_load_by_hash` — Store a blob, then `load(hash)` it. Assert returned
    bytes match exactly.
    - Input: random 4 KiB blob, `hash = ContentHash::from_data(&blob)`
    - Expected: `load(hash) == Ok(Some(blob))`

14. **TC-12.3.2.1** `test_metadata_put_get` — Put metadata for `AssetId(1)`, then `get`. Assert
    every field matches.
    - Input: `AssetMetadata { asset_id: AssetId(1), version: 3, .. }`
    - Expected: `get(AssetId(1))` returns `Some(meta)` where `meta.version == 3`

15. **TC-12.3.3.1** `test_cache_hit_skips_processing` — Import an asset, then re-import unchanged.
    Assert second import returns `ImportResult::CacheHit` and no processor runs.
    - Input: same source bytes + identical settings
    - Expected: `ImportResult::CacheHit { artifact_key }`, processor invocation count == 1

16. **TC-12.3.3.2** `test_cache_miss_on_settings_change` — Re-import same source with changed
    `compression` setting. Assert cache miss and reprocess.
    - Input: settings differ in `TextureCompression::Bc7` → `Astc`
    - Expected: `ImportResult::Imported`, processor invocation count == 2

17. **TC-12.3.4.1** `test_dep_invalidation_transitive` — Three assets `A → B → C` (A depends on B, B
    depends on C). Mark C dirty. Assert `invalidate([C])` returns `[A, B, C]`.
    - Input: dep graph edges `A→B`, `B→C`
    - Expected: returned `Vec<AssetId>` contains `[A, B, C]` in topological order

18. **TC-12.3.5.1** `test_search_full_text` — Index 100 assets with names; query `"sword"`. Assert
    only assets containing `"sword"` in name or tags are returned.
    - Input: 100 metadata entries, 5 with `"sword"` substring
    - Expected: `query(SearchFilter::Text("sword")).len() == 5`

19. **TC-12.3.5.2** `test_search_facet_filter` — Filter by `AssetType::Texture` and tag `"hero"`.
    Assert intersection is returned.
    - Input: 50 assets mixed types; 10 textures, 4 with `"hero"` tag
    - Expected: result length == 4, all entries match both facets

20. **TC-12.3.6.1** `test_thumbnail_async_generated` — Import a mesh; await thumbnail task. Assert
    `metadata.thumbnail_hash` is set and CAS contains the thumbnail blob.
    - Input: cube mesh import with thumbnails enabled
    - Expected: `meta.thumbnail_hash.is_some()`, `cas.exists(thumb_hash) == true`

21. **TC-12.3.10.1** `test_version_history_restore` — Import asset, modify, import again. Restore by
    hash of v1. Assert restored bytes equal v1.
    - Input: v1 bytes, v2 bytes, `restore(v1_hash)`
    - Expected: `version_store.restore(v1_hash) == Ok(v1_bytes)`

22. **TC-12.4.1.1** `test_file_watch_debounce` — Emit 5 modify events for the same file within 100
    ms; debounce window is 200 ms. Assert exactly one `AssetChange` is dispatched.
    - Input: 5 `FileEvent::Modified` events at t=0, 20, 40, 60, 80 ms
    - Expected: `poll_changes()` after 250 ms returns 1 change

23. **TC-12.4.1.2** `test_file_watch_dedupe_rapid` — Modify and rename a file in rapid succession.
    Assert one combined `Renamed` event with the latest path.
    - Input: `Modified("a.png")` then `Renamed("a.png" → "b.png")` within debounce window
    - Expected: 1 event of type `Renamed { from: "a.png", to: "b.png" }`

24. **TC-12.4.2.1** `test_atomic_pointer_swap` — Allocate handle, swap pointer, resolve. Assert new
    pointer is returned and old pointer is queued for retirement.
    - Input: `allocate(p1, 64)` then `swap_ptr(idx, p2, 128)`
    - Expected: `resolve(idx, gen) == Some(p2)`, retirement list contains `p1`

25. **TC-12.4.2.2** `test_swap_dependents_updated` — Texture used by 3 materials. Swap texture.
    Assert all 3 materials see the new texture pointer after swap.
    - Input: 3 materials referencing texture `T` via `AssetHandle<Texture>`
    - Expected: after swap, all 3 `material.tex.get(table)` return the new texture data

26. **TC-12.4.3.1** `test_shader_pso_swap` — Modify shader source; trigger recompile; assert PSO is
    replaced at next frame boundary and old PSO is retired.
    - Input: shader source v1 → v2, `frame_boundary_tick()`
    - Expected: `pso_table[shader_id]` points to v2 PSO, v1 in retirement queue

27. **TC-12.4.3.2** `test_shader_error_overlay` — Recompile shader with syntax error. Assert overlay
    event emitted, previous PSO unchanged.
    - Input: invalid HLSL `float4 main() { return; }`
    - Expected: `ShaderCompileError` event, `pso_table[id]` unchanged from previous good

28. **TC-12.4.4.1** `test_logic_graph_state_preserve` — Hot reload graph with same variable layout.
    Assert variable values are preserved.
    - Input: graph instance with `var_x = 42`; reload with new graph body, same vars
    - Expected: after reload, `instance.read_var("x") == Value::I32(42)`

29. **TC-12.4.4.2** `test_logic_graph_layout_change` — Hot reload graph with new variable added.
    Assert clean restart event and `var_x` reset to default.
    - Input: graph v1 has `var_x`; v2 adds `var_y`
    - Expected: `LogicGraphRestart` event, all variables initialized to defaults

30. **TC-12.4.5.1** `test_ui_subtree_rebuild` — Modify style sheet of one panel; reload. Assert only
    the panel subtree is rebuilt; sibling panels unchanged.
    - Input: 3-panel UI; modify style for panel `B`
    - Expected: panel B's `widget_id` changes, panels A and C `widget_id` unchanged

31. **TC-12.4.6.1** `test_partial_subasset_reimport` — Composite asset with 10 animation clips;
    modify one clip. Assert only that clip is reprocessed.
    - Input: composite with clips `[0..10]`, modify clip `5`
    - Expected: processor invocation count == 1, only clip `5` artifact regenerated

32. **TC-12.4.7.1** `test_editor_runtime_sync` — Editor sends material parameter change. Assert
    runtime applies it within one tick of `EditorSync.poll()`.
    - Input: `EditorMessage::SetMaterialParam { material: M, param: "albedo", value: red }`
    - Expected: after `poll()`, runtime material `M.albedo == [1.0, 0.0, 0.0, 1.0]`

33. **TC-12.7.1.1** `test_asset_header_roundtrip` — Build asset with `AssetWriter`, parse with
    `AssetReader`. Assert magic, version, type ID, hash all match.
    - Input: `AssetWriter::new(asset_type_id=42).add_section("verts", ..).build()`
    - Expected: `reader.header.magic == ASSET_MAGIC`, `reader.header.asset_type_id == 42`,
      `reader.header.format_version == FORMAT_VERSION`

34. **TC-12.7.1.2** `test_asset_section_o1_lookup` — Asset with 100 sections. Assert
    `reader.section("section_50")` performs a single TOC lookup (no scan).
    - Input: 100-section asset; query for section index 50
    - Expected: `section("section_50")` returns the correct slice; benchmark assertion that lookup
      time is independent of section count

35. **TC-12.7.2.1** `test_bundle_partial_decompress` — Bundle of 10 LZ4-compressed assets; extract
    asset index 5 only. Assert other 9 are not decompressed.
    - Input: bundle written with `BundleWriter`, 10 entries, request index 5
    - Expected: returned bytes match asset 5; decoder counter shows 1 chunk decompressed

36. **TC-12.7.3.1** `test_structural_diff_mesh` — Diff two meshes: v1 has 1000 verts, v2 has 1100
    verts. Assert diff reports `vertex_delta = +100`.
    - Input: two `MeshAsset` versions with different vertex counts
    - Expected: `DiffResult::Mesh { vertex_delta: 100, .. }`

37. **TC-12.7.4.1** `test_three_way_merge_disjoint` — Ancestor A; ours adds node N1; theirs adds
    node N2. Assert merge succeeds with both nodes present.
    - Input: 3 logic graph versions with non-overlapping additions
    - Expected: `MergeResult::Success { merged }` and merged graph contains both N1 and N2

38. **TC-12.7.5.1** `test_auto_resolve_lww` — Both branches modify the same scalar property; LWW
    resolves to whichever has the later timestamp. Assert resolution chosen and recorded.
    - Input: ancestor `roughness=0.5`; ours `0.7@t1`; theirs `0.9@t2 > t1`
    - Expected: `AutoResolved { merged: roughness=0.9, resolutions: [LastWriterWins] }`

39. **TC-12.7.5.2** `test_auto_resolve_union` — Both branches add different items to a tag list;
    union strategy combines them.
    - Input: ancestor `tags=[a]`; ours `tags=[a,b]`; theirs `tags=[a,c]`
    - Expected: `AutoResolved { merged: tags=[a,b,c], resolutions: [Union] }`

## Integration Tests

| ID           | Name                              | Req        |
|--------------|-----------------------------------|------------|
| TC-12.1.I.1  | `test_batch_import_100_files`     | R-12.1.5   |
| TC-12.1.I.2  | `test_gltf_full_pipeline`         | R-12.1.1   |
| TC-12.3.I.1  | `test_incremental_rebuild_chain`  | R-12.3.4   |
| TC-12.3.I.2  | `test_search_with_thumbnails`     | R-12.3.5   |
| TC-12.4.I.1  | `test_hot_reload_textured_mesh`   | R-12.4.2   |
| TC-12.4.I.2  | `test_shader_hot_reload_overlay`  | R-12.4.3   |
| TC-12.4.I.3  | `test_editor_sync_roundtrip`      | R-12.4.7   |
| TC-12.7.I.1  | `test_git_merge_driver`           | R-12.7.8   |
| TC-12.7.I.2  | `test_versioning_full_history`    | R-12.3.10  |

1. **TC-12.1.I.1** `test_batch_import_100_files` — Import 100 mixed-format assets (50 PNG, 30 glTF,
   20 WAV) in one batch. Assert all complete with `Imported` status and the database has 100
   entries. Wall time < 10 s on reference hardware.
   - Input: 100 source files spanning three importer types
   - Expected: `BatchReport { imported: 100, failed: 0 }`, `metadata.query(all).len() == 100`, wall
     time < 10 s

2. **TC-12.1.I.2** `test_gltf_full_pipeline` — Import a glTF file with mesh, textures, and a
   skeleton. Assert all sub-assets are registered and dependency edges link mesh → textures and mesh
   → skeleton.
   - Input: glTF 2.0 file with embedded buffers, 2 textures, 1 skeleton
   - Expected: 4 metadata entries, dep graph contains both edges, all assets present in CAS

3. **TC-12.3.I.1** `test_incremental_rebuild_chain` — Three-asset chain
   `Texture → Material → Scene`. Modify texture; trigger rebuild. Assert all three are reprocessed;
   rebuild a fourth unrelated asset is NOT touched.
   - Input: 4 assets (3 in chain, 1 unrelated); modify texture
   - Expected: processor counts: texture=1, material=1, scene=1, unrelated=0

4. **TC-12.3.I.2** `test_search_with_thumbnails` — Import 20 textures, await thumbnail generation,
   then run a faceted search returning textures with thumbnails. Assert all 20 have `thumbnail_hash`
   set and search returns 20 IDs.
   - Input: 20 PNG sources, faceted query `type=Texture, has_thumbnail=true`
   - Expected: 20 results, every result has a CAS-resident thumbnail blob

5. **TC-12.4.I.1** `test_hot_reload_textured_mesh` — Load scene with 1 textured mesh. Modify the
   source PNG. Assert: file watcher fires, importer runs, swap scheduled, runtime texture pointer
   updated within 2 s, no rendering artifacts (handle generation incremented atomically).
   - Input: scene with mesh `M` referencing texture `T`; modify `T`'s PNG file
   - Expected: within 2 s, `T.handle.generation == old + 1`, GPU texture descriptor refreshed

6. **TC-12.4.I.2** `test_shader_hot_reload_overlay` — Load mesh with custom shader. Introduce syntax
   error in shader source. Assert overlay event emitted, previous PSO still bound, then fix shader
   and assert new PSO swapped within one frame boundary.
   - Input: shader source modified to invalid HLSL, then to valid v2
   - Expected: overlay event after invalid; after valid, `shader.pso == v2`, overlay cleared

7. **TC-12.4.I.3** `test_editor_sync_roundtrip` — Editor and runtime connected. Editor changes
   material; runtime moves camera. Assert each side observes the other's update within 100 ms.
   - Input: `EditorSync` with bidirectional channel
   - Expected: runtime material updated within 100 ms; editor camera position updated within 100 ms

8. **TC-12.7.I.1** `test_git_merge_driver` — Two branches modify the same logic graph in
   non-overlapping regions. Run the engine's merge driver. Assert merged file written, exit code 0,
   and no Git conflict markers.
   - Input: ancestor.har, ours.har, theirs.har for the same logic graph
   - Expected: driver writes merged file; `git merge` reports clean

9. **TC-12.7.I.2** `test_versioning_full_history` — Import asset; modify 5 times. Assert version
   store contains 6 entries; `restore(v3_hash)` returns v3 bytes; structural diff between v3 and v4
   is non-empty.
   - Input: 6 sequential imports of the same asset
   - Expected: `version_store.list(asset_id).len() == 6`, restore matches, diff non-empty

## Benchmarks

| ID            | Benchmark                          | Target    | Req        |
|---------------|------------------------------------|-----------|------------|
| TC-12.1.B.1   | Native binary import (10 MB)       | < 50 ms   | R-12.1.1   |
| TC-12.3.B.1   | CAS store + load (4 KiB blob)      | < 200 µs  | R-12.3.1   |
| TC-12.3.B.2   | Metadata query 100k entries        | < 5 ms    | R-12.3.5   |
| TC-12.4.B.1   | Hot reload texture end-to-end      | < 2 s     | R-12.4.2   |
| TC-12.7.B.1   | mmap asset open + first section    | < 1 ms    | R-12.7.1   |

1. **TC-12.1.B.1** — Import a 10 MB native-format asset (header validate + BLAKE3 + CAS write +
   metadata insert). Wall time end-to-end. Measured with `criterion`.

2. **TC-12.3.B.1** — Round-trip a 4 KiB blob: `cas.store(hash, data)` followed by `cas.load(hash)`.
   Wall time for both calls combined.

3. **TC-12.3.B.2** — Query a metadata store containing 100k entries by tag. Wall time from `query()`
   call to result vector returned.

4. **TC-12.4.B.1** — Hot reload a 2K texture: file write → watcher → importer → CAS write → handle
   swap. Wall time from `fs::write` to handle generation increment.

5. **TC-12.7.B.1** — `AssetReader::from_bytes` over an mmap of a 50-section, 100 MB asset, then
   `reader.section("section_25")`. Wall time from mmap call to first byte of the section resolved.
   Must not allocate (zero-copy).
